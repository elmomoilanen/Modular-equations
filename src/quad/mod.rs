//! Implements a solver for quadratic modular equations.
//!
//! Modular quadratic equations are of the form ax^2 + bx + c = d (mod n) where
//! every coefficient or term is a residue class \[*\] belonging to the ring of
//! integers Z/nZ. Modulo `n` must be a positive integer and strictly larger than one.
//!
//! Solutions x, if any, are given as residue classes \[x\] such that
//! each class is represented by smallest nonnegative integer (modulo n).
//!
use crate::{
    arith::{Arith, CoreArith, SignCast},
    factor::Factors,
    lin::LinEq,
    prime,
    utils::{largest_common_dividing_power_of_two, make_index_combinations},
    Int, UInt,
};

use num::{integer, iter};
use std::collections::HashSet;

/// Type for quadratic equations with unsigned terms only.
///
/// Quadratic modular equations are of the form ax^2 + bx + c = d (mod modu) where
/// coefficients `a`, `b`, `c` and `d` must be nonnegative for this type. Furthermore,
/// the modulo term `modu` must have the same unsigned type as the other terms
/// and strictly larger than one as its value.

#[derive(Debug)]
pub struct QuadEq<T: UInt> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub d: T,
    pub modu: T,
}

/// Type for quadratic equations with unsigned modulo and signed other terms.
///
/// Quadratic modular equations are of the form ax^2 + bx + c = d (mod n) where
/// coefficient `a`, `b`, `c` and `d` are signed for this type. Modulo `modu` must be
/// an unsigned type but compatible to the signed type (same byte count), e.g.
/// unsigned type u32 would be accepted if the signed type is i32. The modulo
/// n must be strictly larger than one as its value.

#[derive(Debug)]
pub struct QuadEqSigned<S: Int, T: UInt> {
    pub a: S,
    pub b: S,
    pub c: S,
    pub d: S,
    pub modu: T,
}

impl<T: 'static + UInt> QuadEq<T> {
    /// Solve quadratic modular equation ax^2 + bx + c = d (mod modu).
    ///
    /// There will be 0 to N solutions x, depending on the equation. The easiest kind
    /// of equations to solve are those with prime modulo and hardest with composite
    /// modulo as the modulo must be then first factorized into its prime factor
    /// representation and after that the equation needs to be solved for every
    /// prime power case separately before combining the final solution using the
    /// Chinese remainder theorem.
    ///
    /// If a % modu == 0 (0 is the smallest nonnegative representative of \[a\]) and
    /// also b % modu == 0, there are no solutions since the variable x vanishes
    /// from the equation.
    ///
    /// If there aren't solutions, None is returned.
    ///
    /// # Examples
    ///
    /// Solve equation x^2 + x + 3 = 11 (mod 41)
    ///
    /// ```
    /// use modular_equations::QuadEq;
    ///
    /// let quad_eq = QuadEq::<u32> {a: 1, b: 1, c: 3, d: 11, modu: 41};
    ///
    /// // There are now two solutions, [9] and [31]
    /// match quad_eq.solve() {
    ///     Some(x) => assert_eq!(x, vec![9, 31]),
    ///     None => assert!(false),
    /// }
    /// ```
    ///
    /// Check whether 3 is a quadratic residue for modulo 17
    ///
    /// ```
    /// use modular_equations::QuadEq;
    ///
    /// let quad_eq = QuadEq::<u8> {a: 1, b: 0, c: 0, d: 3, modu: 17};
    ///
    /// // In this case, 3 is not a quadratic residue
    /// assert_eq!(quad_eq.solve(), None);
    /// ```
    pub fn solve(&self) -> Option<Vec<T>> {
        if self.modu <= T::one() {
            return None;
        }

        let a_is_zero = self.a % self.modu == T::zero();

        if a_is_zero && self.b % self.modu == T::zero() {
            return None;
        }

        if a_is_zero {
            let lin_eq = LinEq {
                a: self.b,
                b: self.c,
                c: self.d,
                modu: self.modu,
            };
            return lin_eq.solve();
        }

        let mut quad = QuadEq { ..*self };

        if quad.c > T::zero() {
            quad.d = T::sub_mod(quad.d, quad.c, quad.modu);
            quad.c = T::zero();
        }

        match prime::is_odd_prime(quad.modu) {
            true if quad.a == T::one() && quad.b == T::zero() => {
                // Solve x^2 = d (mod modu)
                quad.solve_quad_residue_odd_prime_mod()
            }
            true => {
                // It might be possible to convert ax^2 + bx = d (mod modu)
                // to (2ax + b)^2 = b^2 + 4ad which can then be solved in two steps
                quad.solve_quad_simple()
            }
            false => {
                let mut factors = Factors::new(quad.modu);

                factors.factorize();
                // Prime factor repr of `quad.modu`: [(p_1,k_1), ..., (p_n,k_n)] s.t.
                // quad.modu = p_1^k_1 * ... * p_n^k_n holds
                let prm_factor_repr = factors.prime_factor_repr();

                quad.solve_quad_composite_mod(&prm_factor_repr)
            }
        }
    }

    /// Solve equation (2ax + b)^2 = d' (mod modu), where modu is an odd prime
    /// and d' = b^2 + 4a(d - c). For this to work, a must be greater than zero.
    /// First solve z^2 = d (mod modu), and then 2ax + b = z (mod modu) for x.
    fn solve_quad_simple(&self) -> Option<Vec<T>> {
        if self.a == T::zero() && self.b == T::zero() {
            return None;
        }
        if self.a % self.modu == T::zero() {
            return self.solve_linear_singular();
        }

        let b2 = T::mult_mod(self.b, self.b, self.modu);
        let four_ad = T::mult_mod(4.into(), T::mult_mod(self.a, self.d, self.modu), self.modu);

        let quad = QuadEq {
            a: self.a,
            b: self.b,
            c: self.c,
            d: T::add_mod_unsafe(b2, four_ad, self.modu),
            modu: self.modu,
        };

        let z = match quad.solve_quad_residue_odd_prime_mod() {
            Some(z) if !z.is_empty() => z,
            _ => return None,
        };

        let mut lin_eq = LinEq {
            a: T::mult_mod(2.into(), quad.a, quad.modu),
            b: quad.b,
            c: z[0],
            modu: quad.modu,
        };

        let mut x_sols = match lin_eq.solve() {
            Some(sols) => sols,
            _ => return None,
        };

        if z[0] == T::zero() || z.len() == 1 {
            // z^2 = d (mod modu) has only one root
            return Some(x_sols);
        }

        lin_eq.c = z[1];

        let mut x_sols_2 = match lin_eq.solve() {
            Some(sols) => sols,
            _ => return Some(x_sols),
        };

        x_sols.append(&mut x_sols_2);
        x_sols.sort();

        Some(x_sols)
    }

    fn solve_linear_singular(&self) -> Option<Vec<T>> {
        if self.b == T::zero() && self.d == T::zero() {
            // a > 0 but a % modu == 0
            return Some(vec![T::zero()]);
        }

        let gcd_bm = T::gcd_mod(self.b, self.modu);

        if self.d % gcd_bm > T::zero() {
            return None;
        }

        if gcd_bm == T::one() {
            Some(vec![T::mult_mod(
                T::multip_inv(self.b, self.modu),
                self.d,
                self.modu,
            )])
        } else {
            let new_modu = self.modu / gcd_bm;
            let base_sol = T::mult_mod(
                T::multip_inv(self.b / gcd_bm, new_modu),
                self.d / gcd_bm,
                new_modu,
            );

            Some(iter::range_step(base_sol, self.modu, new_modu).collect())
        }
    }

    /// Solve equation x^2 = d (mod modu), where modu is an odd prime.
    /// There will be 0 to 2 roots for the equation.
    fn solve_quad_residue_odd_prime_mod(&self) -> Option<Vec<T>> {
        if self.d == T::zero() {
            return Some(vec![self.d]);
        }

        if T::exp_mod(self.d, (self.modu - T::one()) / 2.into(), self.modu) != T::one() {
            // Doesn't satisfy Euler's criterion
            return None;
        }

        match QuadEq::tonelli_shanks(self.d, self.modu) {
            None => None,
            Some(x) if x == T::zero() => Some(vec![x]),
            Some(x) => {
                let mut x_sols = vec![x, T::sub_mod_unsafe(T::zero(), x, self.modu)];
                x_sols.sort();

                Some(x_sols)
            }
        }
    }

    fn tonelli_shanks(q: T, modu: T) -> Option<T> {
        let modu_half = (modu - T::one()) / 2.into();

        let non_resid = match iter::range(2.into(), modu)
            .find(|&b| T::exp_mod_unsafe(b, modu_half, modu) != T::one())
        {
            Some(non_residue) => non_residue,
            None => return None,
        };

        let modu_ev = modu - T::one();
        let pow = modu_ev.trailing_zeros();
        let modu_odd = modu_ev.unsigned_shr(pow);

        let mut par_c = T::exp_mod_unsafe(non_resid, modu_odd, modu);
        let mut par_t = T::exp_mod(q, modu_odd, modu);
        let mut res = T::exp_mod(q, (modu_odd + T::one()) / 2.into(), modu);

        // pow < 128 => m < 128
        let modu_u128: u128 = modu.into();
        let mut m = (pow as u128) % modu_u128;

        loop {
            if par_t == T::zero() {
                break Some(par_t);
            }
            if par_t == T::one() {
                break Some(res);
            }

            let (mut least_i, mut pow_i) = (0, 1);

            while pow_i < m {
                let ex = (1 << pow_i) % modu_u128;
                if T::exp_mod_unsafe_u128(par_t, ex, modu) == T::one() {
                    least_i = pow_i;
                    break;
                }
                pow_i += 1;
            }

            if least_i == 0 {
                // q isn't quadratic residue
                break None;
            }

            let ex = (1 << (m - least_i - 1)) % modu_u128;
            let par_b = T::exp_mod_unsafe_u128(par_c, ex, modu);

            m = least_i;
            par_c = T::mult_mod_unsafe(par_b, par_b, modu);
            par_t = T::mult_mod_unsafe(par_t, par_c, modu);
            res = T::mult_mod_unsafe(res, par_b, modu);
        }
    }

    /// Solve equation ax^2 + bx = d (mod modu) for a composite modulo, where
    /// `factor_repr`: \[(p_1,k_1), ..., (p_n,k_n)\] is its prime factor
    /// representation. Hence, the equation is actually solved in every ring
    /// of integers modulo p_i^k_i and at the end all the solutions are combined
    /// to a final solution for the original composite modulo.
    fn solve_quad_composite_mod(&self, factor_repr: &[(T, u8)]) -> Option<Vec<T>> {
        let mut x_sols: Vec<(T, T)> = vec![];
        let mut x_sols_count = 0;

        let uniq_factors = factor_repr.len();

        let mut modu_start_index: Vec<usize> = vec![0];
        let mut modu_sol_count: Vec<usize> = vec![];

        let mut quad = QuadEq { ..*self };

        for (prm_factor, prm_k) in factor_repr.iter() {
            let total_modulo = (*prm_factor).pow((*prm_k).into());
            quad.modu = *prm_factor;

            let x_sub_sols = if quad.modu > 2.into() {
                match quad.solve_quad_simple() {
                    Some(x_sols) if *prm_k <= 1 => Some(x_sols),
                    Some(x_sols) => quad.lift_with_hensel_method(x_sols, *prm_k),
                    None => None,
                }
            } else {
                quad.solve_quad_mod_power_of_two(*prm_k, total_modulo)
            };

            match x_sub_sols {
                Some(sub_sols) if !sub_sols.is_empty() => {
                    let sub_sol_count = sub_sols.len();
                    modu_sol_count.push(sub_sol_count);

                    for x_sol in sub_sols.iter() {
                        x_sols.push((*x_sol, total_modulo));
                    }

                    x_sols_count += sub_sol_count;
                    modu_start_index.push(x_sols_count);
                }
                _ => return None,
            }
        }

        if uniq_factors > 1 {
            // Multiple factors, combine solutions for the original modulo
            modu_start_index.pop(); // Last index is always redundant

            Some(QuadEq::combine_solution_for_compo_modu(
                x_sols,
                self.modu,
                modu_start_index,
                modu_sol_count,
            ))
        } else {
            // Only one factor (p_i^k_i), nothing to combine
            let mut sol: Vec<T> = x_sols.iter().map(|&x_tuple| x_tuple.0).collect();
            sol.sort();

            Some(sol)
        }
    }

    /// Solve equation ax^2 + bx = d (mod 2^m) for some m >= 1.
    fn solve_quad_mod_power_of_two(&self, prm_k: u8, total_modulo: T) -> Option<Vec<T>> {
        if self.b == T::zero() {
            return self.solve_quad_residue_power_of_two_mod(prm_k, total_modulo);
        }

        // Take out common powers of two if possible
        let t = largest_common_dividing_power_of_two(
            (self.a % total_modulo).into(),
            (self.b % total_modulo).into(),
            (self.d % total_modulo).into(),
        );
        let m_prm_k = prm_k - t; // Always >= 0

        let mut m_quad = QuadEq { ..*self };
        m_quad.a = m_quad.a.unsigned_shr(t.into());
        m_quad.b = m_quad.b.unsigned_shr(t.into());
        m_quad.d = m_quad.d.unsigned_shr(t.into());

        let a_even = m_quad.a & T::one() == T::zero();
        let b_even = m_quad.b & T::one() == T::zero();

        if m_quad.d & T::one() == T::one() && (a_even && b_even || !a_even && !b_even) {
            return None;
        }

        let simple_sols = match m_quad.search_possible_solutions_mod_power_of_two() {
            Some(sols) => sols,
            _ => return None,
        };

        let simple_sols = if prm_k > 1 {
            match m_quad.lift_with_hensel_method(simple_sols, m_prm_k) {
                Some(sols) if t == 0 => return Some(sols),
                Some(sols) => sols,
                _ => return None,
            }
        } else {
            simple_sols
        };

        self.scale_possible_solutions_mod_power_of_two(simple_sols, prm_k, m_prm_k, t)
    }

    /// Solve equation ax^2 = d (mod 2^m) for some m >= 1.
    fn solve_quad_residue_power_of_two_mod(&self, prm_k: u8, total_modulo: T) -> Option<Vec<T>> {
        match prm_k {
            1 => self.solve_quad_simple_mod_two(),
            2 => self.solve_quad_simple_mod_four(total_modulo),
            _ if T::gcd_mod(self.a, total_modulo) == T::one() => {
                self.solve_quad_simple_mod_higher_power_of_two(prm_k, total_modulo)
            }
            _ if self.d & T::one() == T::zero()
                && self.a & T::one() == T::zero()
                && self.a % total_modulo > T::zero() =>
            {
                // a and d even, a not divisible by total modulo (2^prm_k)
                self.solve_quad_simple_even_terms_mod_higher_power_of_two(prm_k, total_modulo)
            }
            _ => None,
        }
    }

    fn solve_quad_simple_mod_two(&self) -> Option<Vec<T>> {
        match (self.a & T::one() == T::one(), self.d & T::one() == T::one()) {
            (true, true) => Some(vec![T::one()]),
            (true, false) => Some(vec![T::zero()]),
            (false, true) => {
                // a even and d odd => no solution
                None
            }
            (false, false) => Some(vec![T::zero(), T::one()]),
        }
    }

    fn solve_quad_simple_mod_four(&self, total_modulo: T) -> Option<Vec<T>> {
        let d_is_even = self.d & T::one() == T::zero();

        if d_is_even {
            let d_div_by_four = self.d % 4.into() == T::zero();
            let a_mod_four = self.a % 4.into();

            if d_div_by_four && a_mod_four == T::zero() {
                Some(vec![T::zero(), T::one(), 2.into(), 3.into()])
            } else if d_div_by_four {
                Some(vec![T::zero(), 2.into()])
            } else if a_mod_four == 2.into() {
                Some(vec![T::one(), 3.into()])
            } else {
                None
            }
        } else if T::gcd_mod(self.a, total_modulo) == T::one() {
            let d = T::mult_mod(T::multip_inv(self.a, total_modulo), self.d, total_modulo);

            if d % 4.into() == T::one() {
                Some(vec![T::one(), 3.into()])
            } else {
                None
            }
        } else {
            None
        }
    }

    fn solve_quad_simple_mod_higher_power_of_two(
        &self,
        prm_k: u8,
        total_modulo: T,
    ) -> Option<Vec<T>> {
        let d = T::mult_mod(T::multip_inv(self.a, total_modulo), self.d, total_modulo);

        if d == T::zero() {
            let step = self.modu.pow((prm_k as f64 / 2f64).ceil() as u32);
            return Some(iter::range_step(T::zero(), total_modulo, step).collect());
        }

        if d % 8.into() == T::one() {
            // Odd squares
            let mut sols: Vec<T> = vec![];
            let base: Vec<T> = vec![T::one(), 3.into()];

            for b in base.into_iter() {
                let mut s = b;

                for j in 3..prm_k {
                    let t = T::pow(2.into(), j.into()); // pow < total_modulo, for every j
                    let s2 = T::mult_mod(s, s, total_modulo);

                    let r = if s2 >= d { (s2 - d) / t } else { (d - s2) / t };

                    s = T::add_mod(s, (r & T::one()) * (t.unsigned_shr(1)), total_modulo);
                }
                sols.push(s);
                sols.push(total_modulo - s);
            }
            return Some(sols);
        }

        let d_sqrt = integer::sqrt(d);

        if T::trunc_square(d_sqrt) == d {
            // Even square, base solution for mod 2 equals 0
            let mut m_quad = QuadEq { ..*self };
            m_quad.a = T::one();
            m_quad.d = d;

            return m_quad.lift_with_hensel_method(vec![T::zero()], prm_k);
        }

        None
    }

    fn solve_quad_simple_even_terms_mod_higher_power_of_two(
        &self,
        prm_k: u8,
        total_modulo: T,
    ) -> Option<Vec<T>> {
        let t = largest_common_dividing_power_of_two(
            (self.a % total_modulo).into(),
            total_modulo.into(),
            (self.d % total_modulo).into(),
        );
        let m_prm_k = prm_k - t; // always >= 0

        let mut m_quad = QuadEq { ..*self };
        m_quad.a = m_quad.a.unsigned_shr(t.into());
        m_quad.d = m_quad.d.unsigned_shr(t.into());
        // Either a or d should be odd now

        let m_total_modulo = self.modu.pow(m_prm_k.into());

        match m_quad.solve_quad_residue_power_of_two_mod(m_prm_k, m_total_modulo) {
            Some(sols) => self.scale_possible_solutions_mod_power_of_two(sols, prm_k, m_prm_k, t),
            _ => None,
        }
    }

    fn search_possible_solutions_mod_power_of_two(&self) -> Option<Vec<T>> {
        let mut sols: Vec<T> = vec![];
        let sols_cand: Vec<T> = vec![T::zero(), T::one()];

        for s in sols_cand.into_iter() {
            let poly_lhs = T::add_mod_unsafe(
                T::mult_mod(self.a, s * s, self.modu),
                T::mult_mod(self.b, s, self.modu),
                self.modu,
            );

            if T::sub_mod(poly_lhs, self.d, self.modu) == T::zero() {
                sols.push(s);
            }
        }

        if sols.is_empty() {
            None
        } else {
            Some(sols)
        }
    }

    fn scale_possible_solutions_mod_power_of_two(
        &self,
        sub_sols: Vec<T>,
        prm_k: u8,
        m_prm_k: u8,
        t: u8,
    ) -> Option<Vec<T>> {
        let modulo = self.modu.pow(prm_k.into()); // Original modulo
        let modulo_t = self.modu.pow(t.into()); // >= 1
        let multiplier = self.modu.pow(m_prm_k.into());

        let mut sols = HashSet::new();

        for s in sub_sols.iter() {
            let mut r = T::zero();

            while r < modulo_t {
                sols.insert(T::add_mod(*s, T::mult_mod(r, multiplier, modulo), modulo));
                r = r + T::one();
            }
        }

        if sols.is_empty() {
            None
        } else {
            Some(Vec::from_iter(sols))
        }
    }

    /// Lift a root x of the quadratic polynomial f(x) = 0 (mod prm^k-1) to
    /// a new root x_new of the same quadratic polynomial but with a larger
    /// modulo prm^k. Lifting fails to produce a unique new root if derivative
    /// of the polynomial evaluated at the root x doesn't got the multiplicative
    /// inverse. In this case, lifting method for singular roots is tried.
    ///
    /// Notice that `self.modu` is expected to be the prime factor prm and arg
    /// `prm_k` determines the final prime power prm^k of the lifting.
    fn lift_with_hensel_method(&self, sub_sols: Vec<T>, prm_k: u8) -> Option<Vec<T>> {
        let mut sols: Vec<T> = vec![];

        for sub_sol in sub_sols.into_iter() {
            let poly_d = T::add_mod(
                T::mult_mod(2.into(), T::mult_mod(self.a, sub_sol, self.modu), self.modu),
                self.b,
                self.modu,
            );

            if T::gcd_mod(self.modu, poly_d) != T::one() {
                // Singular root, poly_d doesn't have multiplicative inverse
                if let Some(mut lifted_sols) = self.lift_singular_root(sub_sol, prm_k) {
                    sols.append(&mut lifted_sols);
                }
                continue;
            }

            let t = T::multip_inv(poly_d, self.modu);

            let mut modu = self.modu;
            let mut lifted_sol = sub_sol;

            for _ in 1..prm_k {
                modu = modu * self.modu;

                let ax = T::mult_mod(self.a, T::mult_mod(lifted_sol, lifted_sol, modu), modu);
                let bx = T::mult_mod(self.b, lifted_sol, modu);
                let cx = T::sub_mod(T::zero(), self.d, modu);

                // poly = a * x_lifted^2 + b * x_lifted + c, where in this case c=-d
                let poly = T::add_mod_unsafe(T::add_mod_unsafe(ax, bx, modu), cx, modu);
                lifted_sol = T::sub_mod_unsafe(lifted_sol, T::mult_mod_unsafe(poly, t, modu), modu);
            }

            sols.push(lifted_sol);
        }

        if sols.is_empty() {
            None
        } else {
            Some(sols)
        }
    }

    fn lift_singular_root(&self, sub_sol: T, prm_k: u8) -> Option<Vec<T>> {
        let mut modu = self.modu;

        let mut sols = vec![sub_sol];

        for _ in 1..prm_k {
            modu = modu * self.modu;

            let mut lifted_sols = vec![];

            for sol in sols.iter() {
                let ax = T::mult_mod(self.a, T::mult_mod(*sol, *sol, modu), modu);
                let bx = T::mult_mod(self.b, *sol, modu);
                let cx = T::sub_mod(T::zero(), self.d, modu);

                let poly = T::add_mod_unsafe(T::add_mod_unsafe(ax, bx, modu), cx, modu);

                if poly % modu == T::zero() {
                    // Every lifting of root `sol`, `sol + t * modu_prev`, is a root of modulo `modu`
                    let modu_prev = modu / self.modu;

                    for new_sol in iter::range_step(*sol, modu, modu_prev) {
                        lifted_sols.push(new_sol);
                    }
                }
            }

            sols = lifted_sols;

            if sols.is_empty() {
                return None;
            }
        }

        Some(sols)
    }

    fn combine_solution_for_compo_modu(
        all_sols: Vec<(T, T)>,
        compo_modu: T,
        modu_start_indices: Vec<usize>,
        modu_sol_counts: Vec<usize>,
    ) -> Vec<T> {
        let mut sols: Vec<T> = vec![];

        let index_combinations = match make_index_combinations(&modu_sol_counts) {
            Some(combi) => combi,
            None => {
                // Should never end up here if program logic ok
                panic!(
                    "Failed to combine a solution for a quadratic equation with composite modulo."
                );
            }
        };

        for combi in index_combinations {
            let mut sum = T::zero();

            for (i, c_i) in combi.iter().enumerate() {
                let idx = *c_i + modu_start_indices[i];

                let modu_div = compo_modu / all_sols[idx].1;
                let inv = T::multip_inv(modu_div, all_sols[idx].1);
                let res = T::mult_mod_unsafe(
                    T::mult_mod(all_sols[idx].0, modu_div, compo_modu),
                    inv,
                    compo_modu,
                );

                sum = T::add_mod_unsafe(sum, res, compo_modu);
            }

            sols.push(sum);
        }
        sols.sort_unstable();

        sols
    }
}

impl<T, S> QuadEqSigned<S, T>
where
    S: Int + SignCast<S, T>,
    T: 'static + UInt + TryFrom<S>,
{
    /// Solve quadratic modular equation for signed type terms.
    ///
    /// This method will try to cast the signed type coefficients to unsigned
    /// type such that after the cast they will represent the smallest nonnegative
    /// integers of their corresponding residue classes. If some of the casts
    /// fails, this method will return None but this should only occur for
    /// S::min_value() value of the signed type S.
    ///
    /// After the cast to unsigned type, `solve` method of the struct `QuadEq` is
    /// called to solve the equation.
    ///
    /// Please see the documentation of `QuadEq` for examples.
    pub fn solve(&self) -> Option<Vec<T>> {
        let a_us = match S::cast_to_unsigned(self.a, self.modu) {
            Some(a) => a,
            None => {
                // Arg `a` cannot be casted to unsigned. Is it S::min_value()?
                return None;
            }
        };

        let b_us = match S::cast_to_unsigned(self.b, self.modu) {
            Some(b) => b,
            None => {
                // Arg `b` cannot be casted to unsigned. Is it S::min_value()?
                return None;
            }
        };

        let c_us = match S::cast_to_unsigned(self.c, self.modu) {
            Some(c) => c,
            None => {
                // Arg `c` cannot be casted to unsigned. Is it S::min_value()?
                return None;
            }
        };

        let d_us = match S::cast_to_unsigned(self.d, self.modu) {
            Some(d) => d,
            None => {
                // Arg `d` cannot be casted to unsigned. Is it S::min_value()?
                return None;
            }
        };

        let quad_eq = QuadEq {
            a: a_us,
            b: b_us,
            c: c_us,
            d: d_us,
            modu: self.modu,
        };

        quad_eq.solve()
    }
}

#[cfg(test)]
mod tests;
