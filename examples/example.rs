struct Model;
impl Model {
    fn to_widget(model: &Model) -> impl ui::Widget<Model> {
        ui::flex!(vertical {
            label1: (ui::widgets::flex::ItemSettings::Fixed, ui::widgets::label::Label::new("boo".to_string(), ui::Fonts::text_font, 16)),
            label2: (ui::widgets::flex::ItemSettings::Fixed, ui::widgets::label::Label::new("boo2".to_string(), ui::Fonts::text_font, 16)),
            label3: (ui::widgets::flex::ItemSettings::Fixed, ui::widgets::label::Label::new("boo3".to_string(), ui::Fonts::text_font, 16)),
            label4: (ui::widgets::flex::ItemSettings::Fixed, ui::widgets::label::Label::new("boo4".to_string(), ui::Fonts::text_font, 16)),
            label5: (ui::widgets::flex::ItemSettings::Fixed, ui::widgets::label::Label::new("boo5".to_string(), ui::Fonts::text_font, 16)),
            label6: (ui::widgets::flex::ItemSettings::Fixed, ui::widgets::label::Label::new("boo6".to_string(), ui::Fonts::text_font, 16)),
            label7: (
                ui::widgets::flex::ItemSettings::Flex(2.0),
                ui::widgets::expand::Expand::new(ui::widgets::label::Label::new("boo7".to_string(), ui::Fonts::text_font, 16))
            ),
            label8: (
                ui::widgets::flex::ItemSettings::Flex(2.0),
                ui::widgets::center::Center::new(ui::widgets::label::Label::new("boo8".to_string(), ui::Fonts::text_font, 16))
            ),
            label9: (ui::widgets::flex::ItemSettings::Fixed, ui::widgets::label::Label::new("boo9".to_string(), ui::Fonts::text_font, 16)),
            label10: (ui::widgets::flex::ItemSettings::Fixed, ui::widgets::label::Label::new("boo10".to_string(), ui::Fonts::text_font, 16)),
            label11: (ui::widgets::flex::ItemSettings::Fixed, ui::widgets::label::Label::new("boo11".to_string(), ui::Fonts::text_font, 16)),
            label12: (ui::widgets::flex::ItemSettings::Fixed, ui::widgets::label::Label::new("boo12".to_string(), ui::Fonts::text_font, 16)),
            label13: (ui::widgets::flex::ItemSettings::Fixed, ui::widgets::label::Label::new("boo13".to_string(), ui::Fonts::text_font, 16)),
        })
    }
}

fn main() {
    ui::run("example", (200, 200), Model, Model::to_widget);
}
