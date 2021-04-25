use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Element<E> {
    pub object: E,
    expiry: Option<Instant>
}



impl<E> Element<E> {
    pub fn new(object: E, element_duration: Option<Duration>) -> Self {
        let expiry = element_duration.map(|duration| Instant::now() + duration);
        Element {object, expiry }
    }

    pub fn expired(&self) -> bool {
        self.expiry
        .map(|expiry| expiry < Instant::now())
        .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use crate::element::Element;
    use std::time::Duration;

    const OBJECT: &str = "OBJECT";

    #[async_std::test]
    async fn not_expired_when_duration_is_none() {
        let item = Element::new(OBJECT, None);
        assert_eq!(item.expired(), false);
    }

    #[async_std::test]
    async fn expired_when_duration_is_zero() {
        let item = Element::new(OBJECT, Some(Duration::new(0, 0)));
        assert_eq!(item.expired(), true);
    }
}