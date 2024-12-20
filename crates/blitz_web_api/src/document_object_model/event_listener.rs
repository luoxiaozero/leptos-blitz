pub trait EventListener<T> {
    fn handle_event(&self, event: T);
}

impl<T, F> EventListener<T> for F
where
    F: Fn(T),
{
    fn handle_event(&self, event: T) {
        self(event)
    }
}
