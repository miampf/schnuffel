pub mod graph;

pub trait ViewState: Default {
    type UpdateType;

    fn update_state(&mut self, new: Self::UpdateType);
}
