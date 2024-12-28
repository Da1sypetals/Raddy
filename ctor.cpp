/// Default constructor, copy, move, assignment
Scalar() = default;
Scalar(const Scalar& _rhs) = default;
Scalar(Scalar&& _rhs) = default;
Scalar& operator=(const Scalar& _rhs) = default;
Scalar& operator=(Scalar&& _rhs) = default;

/// Passive variable a.k.a. constant.
/// Gradient and Hessian are zero.
Scalar(PassiveT _val)
    : val(_val)
{
    static_assert(!dynamic_mode_, "Implicit constructor is only available in static mode. Either choose k at runtime or use make_passive(val, k_dynamic).");
}

/// Active variable.
///     _idx: index in variable vector
Scalar(PassiveT _val, Eigen::Index _idx)
    : val(_val)
{
    static_assert(!dynamic_mode_, "This constructor is only available in static mode. Either choose k at compile time or use make_active(val, idx, k_dynamic).");

    TINYAD_ASSERT_GEQ(_idx, 0);
    TINYAD_ASSERT_L(_idx, k);
    grad(_idx) = 1.0;
}

/// Initialize scalar with known derivatives
static Scalar known_derivatives(PassiveT _val, const GradType& _grad, const HessType& _Hess)
{
    Scalar res;
    res.val = _val;
    res.grad = _grad;

    if constexpr (with_hessian)
        res.Hess = _Hess;

    return res;
}

/// Initialize scalar with known derivatives (univariate case)
static Scalar known_derivatives(PassiveT _val, PassiveT _grad, PassiveT _Hess)
{
    static_assert(k == 1 || dynamic_mode_, "Constructor only available for univariate case. Call overload with vector-valued arguments.");

    Scalar res;
    res.val = _val;
    res.grad = GradType::Constant(1, _grad);

    if constexpr (with_hessian)
        res.Hess = HessType::Constant(1, 1, _Hess);

    return res;
}

/// Initialize passive variable a.k.a. constant with zero derivatives of size _k_dynamic.
/// Only necessary in dynamic mode to pass derivative size at run time.
/// In static mode, use the Scalar(val) constructor instead.
static Scalar make_passive(PassiveT _val, Eigen::Index _k_dynamic)
{
    if constexpr (!dynamic_mode_)
        return Scalar(_val);
    else
    {
        Scalar res;
        res.val = _val;
        res.grad = GradType::Zero(_k_dynamic);

        if constexpr (with_hessian)
            res.Hess = HessType::Zero(_k_dynamic, _k_dynamic);

        return res;
    }
}

/// Initialize active variable with derivatives of size _k_dynamic.
/// Only necessary in dynamic mode to pass derivative size at run time.
/// In static mode, use the Scalar(val, idx) constructor instead.
static Scalar make_active(PassiveT _val, Eigen::Index _idx, Eigen::Index _k_dynamic)
{
    if constexpr (!dynamic_mode_)
        return Scalar(_val, _idx);
    else
    {
        TINYAD_ASSERT_L(_idx, _k_dynamic);

        Scalar res;
        res.val = _val;
        res.grad = GradType::Zero(_k_dynamic);
        res.grad[_idx] = 1.0;

        if constexpr (with_hessian)
            res.Hess = HessType::Zero(_k_dynamic, _k_dynamic);

        return res;
    }
}

/// Initialize an active variable vector of size k from given values.
static Eigen::Matrix<Scalar, k, 1> make_active(
        const Eigen::Matrix<PassiveT, Eigen::Dynamic, 1>& _passive)
{
    if constexpr (dynamic_mode_)
    {
        const Eigen::Index k_dynamic = _passive.size();
        Eigen::Matrix<Scalar, Eigen::Dynamic, 1> active(k_dynamic);
        for (Eigen::Index i = 0; i < k_dynamic; ++i)
            active[i] = Scalar::make_active(_passive[i], i, k_dynamic);

        return active;
    }
    else
    {
        TINYAD_ASSERT_EQ(_passive.size(), k);
        Eigen::Matrix<Scalar, k, 1> active(k);
        for (Eigen::Index i = 0; i < k; ++i)
            active[i] = Scalar(_passive[i], i);

        return active;
    }
}

/// Initialize an active variable vector of size k from given values.
static Eigen::Matrix<Scalar, k, 1> make_active(
        std::initializer_list<PassiveT> _passive)
{
    return make_active(Eigen::Map<const Eigen::Matrix<PassiveT, Eigen::Dynamic, 1>>(_passive.begin(), _passive.size()));
}