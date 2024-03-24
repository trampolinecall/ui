// TODO: make a macro to just make the struct
#[macro_export]
macro_rules! flex {
    (horizontal $($rest:tt)*) => {
        $crate::flex!($crate::widgets::flex::Direction::Horizontal $($rest)*)
    };
    (vertical $($rest:tt)*) => {
        $crate::flex!($crate::widgets::flex::Direction::Vertical $($rest)*)
    };
    ($direction:path { $( $name:ident : ( $settings:expr, $e:expr ) ),* $(,)? }) => {
        {
            #[allow(non_camel_case_types)]
            struct Container<Data, $($name: $crate::widgets::Widget<Data>),*> {
                $(
                    $name: ($crate::widgets::flex::ItemSettings, $name),
                )*
                _phantom: ::std::marker::PhantomData<fn(&mut Data)>,
            }
            #[allow(non_camel_case_types)]
            struct ContainerActualWidget<Data, $($name: $crate::actual_widget::ActualWidget<Data>),*> {
                own_size: $crate::graphics::Vector2f,
                $(
                    $name: ($crate::actual_widget::animated::Animated<$crate::widgets::flex::ItemSettings>, $crate::graphics::Vector2f, $name),
                )*

                _phantom: ::std::marker::PhantomData<fn(&mut Data)>,
            }

            #[allow(non_camel_case_types)]
            impl<Data, $($name: $crate::widgets::Widget<Data>),*> $crate::widgets::Widget<Data> for Container<Data, $($name),*> {
                type ActualWidget = ContainerActualWidget<Data, $(<$name as $crate::widgets::Widget<Data>>::ActualWidget),*>;

                fn to_actual_widget(self, id_maker: &mut $crate::actual_widget::ActualWidgetIdMaker) -> Self::ActualWidget {
                    ContainerActualWidget {
                        own_size: $crate::graphics::Vector2f::new(0.0, 0.0),
                        $(
                            $name: ($crate::actual_widget::animated::Animated::new(self.$name.0), $crate::graphics::Vector2f::new(0.0, 0.0), self.$name.1.to_actual_widget(id_maker)),
                        )*
                        _phantom: ::std::marker::PhantomData,
                    }
                }

                fn update_actual_widget(self, actual_widget: &mut Self::ActualWidget, id_maker: &mut $crate::actual_widget::ActualWidgetIdMaker) {
                    $(
                        actual_widget.$name.0.set(self.$name.0);
                        self.$name.1.update_actual_widget(&mut actual_widget.$name.2, id_maker);
                    )*
                }
            }
            #[allow(non_camel_case_types)]
            impl<Data, $($name: $crate::actual_widget::ActualWidget<Data>),*> $crate::actual_widget::ActualWidget<Data> for ContainerActualWidget<Data, $($name),*> {
                fn layout(&mut self, graphics_context: &$crate::graphics::GraphicsContext, sc: $crate::layout::SizeConstraints) {
                    // lay out fixed elements and count up total flex scaling factors
                    let mut total_flex_scale = 0.0;
                    let mut major_size_left = $direction.take_major_component(sc.max);
                    $(
                        {
                            let (settings, _, ref mut child) = self.$name;
                            $crate::widgets::flex::_layout::first_phase_step(graphics_context, sc, $direction, &mut total_flex_scale, &mut major_size_left, $crate::widgets::flex::_layout::animated_settings(settings), child);
                        }
                    )*

                    // lay out all of the flex children
                    $(
                        {
                            let (settings, _, ref mut child) = self.$name;
                            $crate::widgets::flex::_layout::second_phase_step(graphics_context, sc, $direction, total_flex_scale, major_size_left, $crate::widgets::flex::_layout::animated_settings(settings), child);
                        }
                    )*

                    // assign each of the offsets and calcaulte own_size
                    let mut major_offset = 0.0;
                    let mut max_minor_size = 0.0;
                    $(
                        #[allow(unused_assignments)]
                        {
                            let (_, ref mut offset, ref mut child) = self.$name;
                            *offset = $crate::widgets::flex::_layout::third_phase_step($direction, &mut major_offset, &mut max_minor_size, child);
                        }
                    )*
                    self.own_size = sc.clamp_size($direction.make_vector_in_direction(major_offset, max_minor_size));
                }

                fn draw(&self, graphics_context: &$crate::graphics::GraphicsContext, target: &mut dyn $crate::graphics::RenderTarget, top_left: $crate::graphics::Vector2f, hover: &::std::collections::HashSet<$crate::actual_widget::ActualWidgetId>) {
                    $(
                        {
                            let (_, offset, child) = &self.$name;
                            child.draw(graphics_context, target, top_left + *offset, hover);
                        }
                    )*
                }

                fn find_hover(&self, top_left: $crate::graphics::Vector2f, mouse: $crate::graphics::Vector2f) -> ::std::boxed::Box<dyn ::std::iter::Iterator<Item = ($crate::actual_widget::ActualWidgetId, bool)> + '_> {
                    ::std::boxed::Box::new(
                        ::std::iter::empty()
                            $(
                                .chain({
                                    let (_, offset, child) = &self.$name;
                                    child.find_hover(top_left + *offset, mouse)
                                })
                            )*
                    )
                }

                fn size(&self) -> $crate::graphics::Vector2f {
                    self.own_size
                }

                fn send_targeted_event(&mut self, top_left: $crate::graphics::Vector2f, data: &mut Data, target: $crate::actual_widget::ActualWidgetId, event: $crate::event::TargetedEvent) {
                    $(
                        self.$name.2.send_targeted_event(top_left + self.$name.1, data, target, event);
                    )*
                }

                fn targeted_event(&mut self, _: $crate::graphics::Vector2f, _: &mut Data, _: $crate::event::TargetedEvent) {}
                fn general_event(&mut self, top_left: $crate::graphics::Vector2f, data: &mut Data, event: $crate::event::GeneralEvent) {
                    $(
                        self.$name.2.general_event(top_left + self.$name.1, data, event);
                     )*
                }
            }

            Container {
                $(
                    $name: ($settings, $e),
                )*
                _phantom: ::std::marker::PhantomData,
            }
        }
    };
}
