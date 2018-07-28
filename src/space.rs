use num::Real;
use dimension::Dimension;

// struct MetricProperties<R: Real> {
//     // Making this statically allocated requires usage of
//     // `<Self as T>::R` bounds on the `Dimension` trait, which
//     // is out of the question, as they'd have to be declared
//     // in any function using `Dimension`/`Space` as a generic bound.
//     metric_tensor: DMatrix<R>,
// }

trait Space {
    type Dimension: Dimension;
    type Scalar: Real;

//     fn metric_properties() -> &'static MetricProperties<Self::Scalar>;
}
