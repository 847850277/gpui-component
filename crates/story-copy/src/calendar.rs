use gpui::{App, AppContext, Context, Div, Entity, FocusHandle, Focusable, IntoElement, ParentElement, Render, Styled, Window};
use gpui_component::calendar::Calendar;
use gpui_component::v_flex;
use crate::section::section;

pub struct CalendarStory {
    focus_handle: FocusHandle,
    calendar: Entity<Calendar>,
    calendar_wide: Entity<Calendar>,
}



impl super::Story for CalendarStory{

    fn title() -> &'static str {
        "Calendar"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render + Focusable> {
        Self::view(window, cx)
    }
}


impl CalendarStory {

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let calendar = cx.new(|cx| Calendar::new(window, cx));
        let calendar_wide = cx.new(|cx| Calendar::new(window, cx).number_of_months(3));

        Self {
            calendar,
            calendar_wide,
            focus_handle: cx.focus_handle(),
        }
    }

}


impl Focusable for CalendarStory {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CalendarStory {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(section("Normal").max_w_md().child(self.calendar.clone()))
            .child(
                section("With 3 Months")
                    .max_w_md()
                    .child(self.calendar_wide.clone()),
            )
    }
}