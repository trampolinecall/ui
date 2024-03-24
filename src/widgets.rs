pub mod center;
pub mod clickable;
pub mod either;
pub mod empty;
pub mod expand;
#[macro_use]
pub mod flex;
pub mod fixed_size;
pub mod label;
pub mod max_size;
pub mod min_size;
pub mod padding;
pub(crate) mod responds_to_keyboard; // not finished yet so not exported
pub(crate) mod test_rect;
pub mod vsplit;

use crate::actual_widget::{ActualWidget, ActualWidgetIdMaker};

pub trait Widget<Data: ?Sized> {
    type ActualWidget: ActualWidget<Data>;
    fn to_actual_widget(self, id_maker: &mut ActualWidgetIdMaker) -> Self::ActualWidget;
    fn update_actual_widget(self, actual_widget: &mut Self::ActualWidget, id_maker: &mut ActualWidgetIdMaker);
}
