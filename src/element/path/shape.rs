use vello::kurbo::{self, ParamCurveArclen, ParamCurveArea, ParamCurveExtrema, Shape};

use super::{Path, PathElements};

impl Shape for Path {
    type PathElementsIter<'iter> = KurboPathElements<'iter>;

    fn path_elements(&self, _tolerance: f64) -> Self::PathElementsIter<'_> {
        KurboPathElements { inner: self.iter() }
    }

    fn area(&self) -> f64 {
        kurbo::segments(KurboPathElements { inner: self.iter() })
            .map(|segment| segment.signed_area())
            .sum()
    }

    fn perimeter(&self, accuracy: f64) -> f64 {
        kurbo::segments(KurboPathElements { inner: self.iter() })
            .map(|segment| segment.arclen(accuracy))
            .sum()
    }

    fn winding(&self, pt: kurbo::Point) -> i32 {
        kurbo::segments(KurboPathElements { inner: self.iter() })
            .map(|segment| segment.winding(pt))
            .sum()
    }

    fn bounding_box(&self) -> kurbo::Rect {
        let mut bbox: Option<kurbo::Rect> = None;

        for segment in kurbo::segments(KurboPathElements { inner: self.iter() }) {
            let segment_bbox = ParamCurveExtrema::bounding_box(&segment);

            bbox =
                Some(bbox.map(|bbox| bbox.union(segment_bbox)).unwrap_or(segment_bbox));
        }

        bbox.unwrap_or_default()
    }
}

pub struct KurboPathElements<'a> {
    inner: PathElements<'a>,
}

impl<'a> Iterator for KurboPathElements<'a> {
    type Item = kurbo::PathEl;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(Into::into)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.inner.len(), Some(self.inner.len()))
    }
}

impl ExactSizeIterator for KurboPathElements<'_> {}
