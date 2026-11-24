#[cfg(feature = "translations")]
mod action_bar;
#[cfg(all(feature = "micropython", feature = "translations"))]
mod address_details;
#[cfg(feature = "ui_overlay")]
mod binary_selection;
#[cfg(feature = "bootloader")]
pub mod bl_confirm;
mod button;
#[cfg(feature = "translations")]
mod coinjoin_progress;
#[cfg(not(feature = "bootloader"))]
mod fido;
#[cfg(not(feature = "bootloader"))]
mod footer;
mod vertical_menu;
#[cfg(not(feature = "bootloader"))]
#[rustfmt::skip]
mod fido_icons;
mod error;
#[cfg(not(feature = "bootloader"))]
mod frame;
#[cfg(not(feature = "bootloader"))]
mod header;
#[cfg(feature = "translations")]
mod hold_to_confirm;
#[cfg(feature = "translations")]
mod homescreen;
#[cfg(feature = "translations")]
mod keyboard;
mod loader;
#[cfg(feature = "translations")]
mod more_info_screen;
#[cfg(feature = "translations")]
mod number_input;
#[cfg(feature = "translations")]
pub mod number_input_slider;
mod progress;
#[cfg(feature = "translations")]
mod prompt_screen;
mod result;
#[cfg(feature = "translations")]
mod share_words;
mod status_screen;
mod swipe_content;
#[cfg(feature = "translations")]
mod swipe_up_screen;
#[cfg(feature = "translations")]
mod tap_to_confirm;
mod trade_screen;
mod welcome_screen;

#[cfg(feature = "translations")]
pub use action_bar::{ActionBar, ActionBarMsg};
#[cfg(all(feature = "micropython", feature = "translations"))]
pub use address_details::AddressDetails;
#[cfg(feature = "ui_overlay")]
pub use binary_selection::{BinarySelection, BinarySelectionMsg};
pub use button::{Button, ButtonContent, ButtonMsg, ButtonStyle, ButtonStyleSheet, IconText};
#[cfg(feature = "translations")]
pub use coinjoin_progress::CoinJoinProgress;
pub use error::ErrorScreen;
#[cfg(not(feature = "bootloader"))]
pub use fido::FidoCredential;
#[cfg(not(feature = "bootloader"))]
pub use footer::Footer;
#[cfg(not(feature = "bootloader"))]
pub use frame::{Frame, FrameMsg};
#[cfg(not(feature = "bootloader"))]
pub use header::Header;
#[cfg(feature = "translations")]
pub use hold_to_confirm::HoldToConfirm;
#[cfg(feature = "micropython")]
pub use homescreen::{check_homescreen_format, Homescreen, HomescreenMsg, Lockscreen};
#[cfg(feature = "translations")]
pub use keyboard::{
    bip39::Bip39Input,
    mnemonic::{MnemonicInput, MnemonicKeyboard, MnemonicKeyboardMsg},
    passphrase::{PassphraseKeyboard, PassphraseKeyboardMsg},
    pin::{PinKeyboard, PinKeyboardMsg},
    slip39::Slip39Input,
    word_count::{SelectWordCount, SelectWordCountLayout, SelectWordCountMsg},
};
pub use loader::{Loader, LoaderMsg, LoaderStyle, LoaderStyleSheet};
#[cfg(feature = "translations")]
pub use more_info_screen::MoreInfoScreen;
#[cfg(feature = "translations")]
pub use number_input::{NumberInputDialog, NumberInputDialogMsg};
#[cfg(feature = "translations")]
pub use number_input_slider::NumberInputSliderDialog;
pub use progress::Progress;
#[cfg(feature = "translations")]
pub use prompt_screen::{PromptMsg, PromptScreen};
pub use result::{ResultFooter, ResultScreen, ResultStyle};
#[cfg(feature = "translations")]
pub use share_words::ShareWords;
pub use status_screen::StatusScreen;
pub use swipe_content::{InternallySwipableContent, SwipeContent};
#[cfg(feature = "translations")]
pub use swipe_up_screen::{SwipeUpScreen, SwipeUpScreenMsg};
#[cfg(feature = "translations")]
pub use tap_to_confirm::TapToConfirm;
pub use trade_screen::TradeScreen;
pub use vertical_menu::{
    PagedVerticalMenu, ScrolledVerticalMenu, VerticalMenu, VerticalMenuChoiceMsg, VerticalMenuItem,
    VerticalMenuItems,
};
pub use welcome_screen::WelcomeScreen;

use super::{constant, theme};
