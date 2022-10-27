//! Implements integer factorization.
//!
//! The complete factorization algorithm consists of
//! - Trial division with few of the smallest primes.
//! - Fermat's factorization method, useful if the integer is of the form n=(a+b)*(a-b).
//! - Primality testing, module `prime` implements Miller-Rabin and strong Baillie-PSW tests.
//! - Lenstra elliptic-curve factorization with multiple of worker threads. Module `elliptic`
//! implements elliptic curve arithmetic needed during factorization.
//!
//! Constant `MAX_WORKERS` defines the maximal thread count. This value must be at least two
//! and preferably between three and six (by rough empirical testing). First thread will
//! actually run the wheel factorization targeting smaller prime factors whereas other threads
//! run the actual elliptic-curve factorization method. Thus, if the thread count has been
//! set to one, only the wheel factorization will run.
//!
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use num::integer;

use crate::{arith::Arith, elliptic::EllipticCurve, prime, UInt};

/// Thread count for elliptic curve factorization.
/// Set between 3 and 6 (best efficiency by rough empirical testing).
const MAX_WORKERS: usize = 5;

/// Max count of elliptic curves during single elliptic factorization run.
const MAX_ELLIPTIC_CURVES: usize = 125;

struct MaybeFactors<T: UInt> {
    num: T,
    factors: Vec<(T, bool)>,
}

pub struct Factors<T: UInt> {
    pub num: T,
    pub factors: Vec<T>,
}

impl<T: 'static + UInt> Factors<T> {
    pub fn new(num: T) -> Factors<T> {
        Self {
            num,
            factors: Vec::<T>::new(),
        }
    }

    /// Factor a positive natural number `self.num` to its prime factors.
    ///
    /// After the call, `factors` field of the struct contains
    /// all the prime factors, smallest prime being the first
    /// element in the container. Field `num` remains unmodified.
    ///
    /// Resulted factors can be used to recover the original natural
    /// number `num` via the prime factor representation.
    pub fn factorize(&mut self) {
        if self.num <= T::one() {
            // Should never go here if program logic ok
            panic!("Cannot factorize natural number smaller than two");
        }

        self.factors.clear();

        let num = self.factorize_trial(self.num);

        self.factorize_until_completed(num);

        // Factorize_elliptic step might have resulted extra factors, prune them now
        self.prune_duplicate_factors()
    }

    /// Get the prime factor representation for the natural number `num`:
    /// num = prm_1^k_1 * prm_2^k_2 * ... * prm_n^k_n.
    ///
    /// Representation is returned such that each element of the container
    /// is a tuple with the prime factor `prm_i` and its count `k_i` as
    /// its two elements, ordered s.t. the first tuple has the smallest prime.
    ///
    /// This method assumes that the `factors` field has the correct prime
    /// factors sorted from smallest to largest and as such the representation
    /// can be directly produced from them.
    ///
    /// Hence, always call the `factorize` method prior calling this.
    pub fn prime_factor_repr(&self) -> Vec<(T, u8)> {
        let mut prm_factor_repr = Vec::<(T, u8)>::new();

        let mut k = self.num;
        let mut count = 0;
        let mut prev_factor = T::zero();

        for factor in self.factors.iter().rev() {
            let curr_factor = *factor;

            if curr_factor != prev_factor && count > 0 {
                prm_factor_repr.push((prev_factor, count));
                count = 0;
            }

            count += 1;
            k = k / curr_factor;

            prev_factor = curr_factor;

            if k == T::one() {
                prm_factor_repr.push((prev_factor, count));
                break;
            }
        }

        prm_factor_repr.reverse();

        prm_factor_repr
    }

    fn factorize_until_completed(&mut self, mut num: T) {
        while num > T::one() {
            num = self.factorize_fermat(num, 2);

            if num == T::one() {
                break;
            }

            if prime::is_odd_prime(num) {
                self.factors.push(num);
                break;
            }

            num = self.factorize_elliptic(num);
        }
    }

    fn factorize_trial(&mut self, mut num: T) -> T {
        static PRIMES: [u8; 54] = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251,
        ];

        for prm in PRIMES.iter() {
            let prime = (*prm).into();

            while num % prime == T::zero() {
                self.factors.push(prime);
                num = num / prime;
            }

            if num == T::one() {
                break;
            }
        }

        num
    }

    fn factorize_fermat(&mut self, num: T, level: usize) -> T {
        let mut a = integer::sqrt(num);
        let mut a_square = T::trunc_square(a);

        if a_square == num {
            if prime::is_odd_prime(a) {
                for _ in 0..level {
                    self.factors.push(a);
                }

                return T::one();
            }
            // a not yet prime, start recursive search
            let mut num_back = self.factorize_fermat(a, level << 1);

            if num_back > T::one() {
                // Factoring not completed, return the original num
                num_back = num;
            }
            return num_back;
        }

        a = a + T::one();
        a_square = T::trunc_square(a);

        if a_square == T::zero() {
            return num;
        }

        for _ in 0..10 {
            let b_square = a_square - num;
            let b = integer::sqrt(b_square);

            if T::trunc_square(b) == b_square {
                let rounds = level >> 1;

                for _ in 0..rounds {
                    self.factors.push(a - b);
                    self.factors.push(a + b);
                }

                return T::one();
            }

            a = a + T::one();
            a_square = T::trunc_square(a);

            if a_square == T::zero() {
                return num;
            }
        }

        num
    }

    fn factorize_elliptic(&mut self, mut num: T) -> T {
        let mut ec_factors: Vec<(T, bool)> = Vec::new();

        num = self.spawn_workers(num, &mut ec_factors);

        for (ec_factor, is_sure_prime) in ec_factors {
            if is_sure_prime || prime::is_odd_prime(ec_factor) {
                self.factors.push(ec_factor);
            } else {
                // Factor must be a power of prime or product of several primes
                let mut factors_inner = Factors::new(ec_factor);
                factors_inner.factorize_until_completed(ec_factor);

                for new_factor in factors_inner.factors {
                    self.factors.push(new_factor);
                }
            }
        }

        num
    }

    fn spawn_workers(&self, num: T, factors: &mut Vec<(T, bool)>) -> T {
        let (sender, receiver) = mpsc::channel();

        let maybe_factors_mtx = Arc::new(Mutex::new(MaybeFactors {
            num,
            factors: Vec::new(),
        }));

        for worker in 0..MAX_WORKERS {
            let sender = sender.clone();
            let maybe_factors_mtx_clone = Arc::clone(&maybe_factors_mtx);

            thread::spawn(move || {
                if worker == 0 {
                    // Try to find smaller factors with wheel factorization
                    Self::wheel_worker(maybe_factors_mtx_clone, num, sender);
                } else {
                    Self::elliptic_worker(maybe_factors_mtx_clone, num, sender);
                }
            });
        }

        match receiver.recv() {
            Ok(completed) => {
                let maybe_factors_guard = match maybe_factors_mtx.lock() {
                    Ok(mtx_guard) => mtx_guard,
                    _ => {
                        eprintln!("Error: maybe_factors_mtx.lock() panicked.");
                        return num;
                    }
                };

                for tuple in (*maybe_factors_guard).factors.iter() {
                    factors.push(*tuple);
                }
                if completed {
                    T::one()
                } else {
                    (*maybe_factors_guard).num
                }
            }
            Err(_) => {
                eprintln!("Error: all elliptic workers disconnected, channel closed.");

                let maybe_factors_guard = maybe_factors_mtx.lock().unwrap();

                for tuple in (*maybe_factors_guard).factors.iter() {
                    factors.push(*tuple);
                }

                (*maybe_factors_guard).num
            }
        }
    }

    fn elliptic_worker(
        maybe_factors: Arc<Mutex<MaybeFactors<T>>>,
        mut num: T,
        sender: mpsc::Sender<bool>,
    ) {
        let mut curve_count = 1;

        while num > T::one() && curve_count <= MAX_ELLIPTIC_CURVES {
            let maybe_factor = EllipticCurve::compute_maybe_factor_from_curve(num);

            if maybe_factor > T::one() && maybe_factor < num {
                let mut factors_guard = match maybe_factors.lock() {
                    Ok(mtx_guard) => mtx_guard,
                    _ => {
                        curve_count += 1;
                        continue;
                    }
                };

                if maybe_factor > (*factors_guard).num {
                    num = (*factors_guard).num;
                } else {
                    num = num / maybe_factor;
                    (*factors_guard).num = num;
                    (*factors_guard).factors.push((maybe_factor, false));

                    if prime::is_odd_prime(num) {
                        (*factors_guard).factors.push((num, true));
                        num = T::one();
                        (*factors_guard).num = num;
                    }
                }
            } else if maybe_factor == num && prime::is_odd_prime(maybe_factor) {
                let mut factors_guard = match maybe_factors.lock() {
                    Ok(mtx_guard) => mtx_guard,
                    _ => {
                        curve_count += 1;
                        continue;
                    }
                };

                if maybe_factor == (*factors_guard).num {
                    num = T::one();
                    (*factors_guard).num = num;
                    (*factors_guard).factors.push((maybe_factor, true));
                } else {
                    num = (*factors_guard).num;
                }
            } else if curve_count & 31 == 0 {
                // Update factored number `num`
                if let Ok(mtx_guard) = maybe_factors.lock() {
                    num = (*mtx_guard).num;
                }
            }

            curve_count += 1;
        }

        if sender.send(num == T::one()).is_err() {}
    }

    fn wheel_worker(
        maybe_factors: Arc<Mutex<MaybeFactors<T>>>,
        mut num: T,
        sender: mpsc::Sender<bool>,
    ) {
        // Use basis {2, 3, 5, 7}
        let wheel_inc: [u8; 48] = [
            2, 4, 2, 4, 6, 2, 6, 4, 2, 4, 6, 6, 2, 6, 4, 2, 6, 4, 6, 8, 4, 2, 4, 2, 4, 8, 6, 4, 6,
            2, 4, 6, 2, 6, 6, 4, 2, 4, 6, 2, 6, 4, 2, 4, 2, 10, 2, 10,
        ];

        let mut k = 221.into(); // Start search from 48th prime 223 (221 + first wheel inc)

        for wheel in wheel_inc.iter().cycle() {
            k = k + (*wheel).into();

            if k > num / k {
                if let Ok(mut factors_guard) = maybe_factors.lock() {
                    (*factors_guard).factors.push((num, false));
                    num = T::one();
                    (*factors_guard).num = num;
                }
                break;
            }

            if num % k == T::zero() {
                let mut factors_guard = match maybe_factors.lock() {
                    Ok(mtx_guard) => mtx_guard,
                    _ => break,
                };

                if k > (*factors_guard).num || (*factors_guard).factors.iter().any(|&e| e.0 == k) {
                    // Maybe factor `k` already larger than the active number or it has already been found
                    num = (*factors_guard).num;
                    break;
                }

                loop {
                    num = num / k;

                    (*factors_guard).num = num;
                    (*factors_guard).factors.push((k, true));

                    if num % k != T::zero() {
                        break;
                    }
                }
            }
        }

        if sender.send(num == T::one()).is_err() {}
    }

    fn prune_duplicate_factors(&mut self) {
        self.factors.sort();

        let mut unique_factors: Vec<T> = vec![];
        let mut k = self.num;

        for factor in self.factors.iter().rev() {
            if k % *factor == T::zero() {
                unique_factors.push(*factor);
                k = k / *factor;
            }
        }

        unique_factors.reverse();

        self.factors = unique_factors;
    }
}

#[cfg(test)]
mod tests;
