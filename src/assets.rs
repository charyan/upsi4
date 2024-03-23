use lazy_static::lazy_static;
use macroquad::{
    prelude::ImageFormat,
    text::{load_ttf_font_from_bytes, Font},
    texture::Texture2D,
};

lazy_static! {
    pub static ref EMPLOYEE_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/employees/employee0_normal.png"),
        Some(ImageFormat::Png),
    );
    pub static ref OFFICE_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/office/office.png"),
        Some(ImageFormat::Png),
    );
    pub static ref FRAME_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/gui/qte_frame.png"),
        Some(ImageFormat::Png),
    );
    pub static ref FRAME_MAGENTA_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/gui/qte_frame_red.png"),
        Some(ImageFormat::Png),
    );
    pub static ref FONT: Font =
        load_ttf_font_from_bytes(include_bytes!("../assets/gui/OpenSans-Medium.ttf")).unwrap();
    pub static ref Z_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/gui/z.png"),
        Some(ImageFormat::Png),
    );
    pub static ref X_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/gui/x.png"),
        Some(ImageFormat::Png),
    );
    pub static ref ICON_DOOR: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/gui/icon_door.png"),
        Some(ImageFormat::Png),
    );
    pub static ref ICON_METH: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/gui/icon_meth.png"),
        Some(ImageFormat::Png)
    );
    pub static ref ICON_RH: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/gui/icon_rh.png"),
        Some(ImageFormat::Png)
    );
}
