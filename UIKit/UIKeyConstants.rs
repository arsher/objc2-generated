//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uikeyboardhidusage?language=objc)
// NS_ENUM
#[cfg(feature = "objc2-core-foundation")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIKeyboardHIDUsage(pub CFIndex);
#[cfg(feature = "objc2-core-foundation")]
impl UIKeyboardHIDUsage {
    #[doc(alias = "UIKeyboardHIDUsageKeyboardErrorRollOver")]
    pub const KeyboardErrorRollOver: Self = Self(0x01);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardPOSTFail")]
    pub const KeyboardPOSTFail: Self = Self(0x02);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardErrorUndefined")]
    pub const KeyboardErrorUndefined: Self = Self(0x03);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardA")]
    pub const KeyboardA: Self = Self(0x04);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardB")]
    pub const KeyboardB: Self = Self(0x05);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardC")]
    pub const KeyboardC: Self = Self(0x06);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardD")]
    pub const KeyboardD: Self = Self(0x07);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardE")]
    pub const KeyboardE: Self = Self(0x08);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF")]
    pub const KeyboardF: Self = Self(0x09);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardG")]
    pub const KeyboardG: Self = Self(0x0A);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardH")]
    pub const KeyboardH: Self = Self(0x0B);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardI")]
    pub const KeyboardI: Self = Self(0x0C);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardJ")]
    pub const KeyboardJ: Self = Self(0x0D);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardK")]
    pub const KeyboardK: Self = Self(0x0E);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardL")]
    pub const KeyboardL: Self = Self(0x0F);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardM")]
    pub const KeyboardM: Self = Self(0x10);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardN")]
    pub const KeyboardN: Self = Self(0x11);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardO")]
    pub const KeyboardO: Self = Self(0x12);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardP")]
    pub const KeyboardP: Self = Self(0x13);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardQ")]
    pub const KeyboardQ: Self = Self(0x14);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardR")]
    pub const KeyboardR: Self = Self(0x15);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardS")]
    pub const KeyboardS: Self = Self(0x16);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardT")]
    pub const KeyboardT: Self = Self(0x17);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardU")]
    pub const KeyboardU: Self = Self(0x18);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardV")]
    pub const KeyboardV: Self = Self(0x19);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardW")]
    pub const KeyboardW: Self = Self(0x1A);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardX")]
    pub const KeyboardX: Self = Self(0x1B);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardY")]
    pub const KeyboardY: Self = Self(0x1C);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardZ")]
    pub const KeyboardZ: Self = Self(0x1D);
    #[doc(alias = "UIKeyboardHIDUsageKeyboard1")]
    pub const Keyboard1: Self = Self(0x1E);
    #[doc(alias = "UIKeyboardHIDUsageKeyboard2")]
    pub const Keyboard2: Self = Self(0x1F);
    #[doc(alias = "UIKeyboardHIDUsageKeyboard3")]
    pub const Keyboard3: Self = Self(0x20);
    #[doc(alias = "UIKeyboardHIDUsageKeyboard4")]
    pub const Keyboard4: Self = Self(0x21);
    #[doc(alias = "UIKeyboardHIDUsageKeyboard5")]
    pub const Keyboard5: Self = Self(0x22);
    #[doc(alias = "UIKeyboardHIDUsageKeyboard6")]
    pub const Keyboard6: Self = Self(0x23);
    #[doc(alias = "UIKeyboardHIDUsageKeyboard7")]
    pub const Keyboard7: Self = Self(0x24);
    #[doc(alias = "UIKeyboardHIDUsageKeyboard8")]
    pub const Keyboard8: Self = Self(0x25);
    #[doc(alias = "UIKeyboardHIDUsageKeyboard9")]
    pub const Keyboard9: Self = Self(0x26);
    #[doc(alias = "UIKeyboardHIDUsageKeyboard0")]
    pub const Keyboard0: Self = Self(0x27);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardReturnOrEnter")]
    pub const KeyboardReturnOrEnter: Self = Self(0x28);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardEscape")]
    pub const KeyboardEscape: Self = Self(0x29);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardDeleteOrBackspace")]
    pub const KeyboardDeleteOrBackspace: Self = Self(0x2A);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardTab")]
    pub const KeyboardTab: Self = Self(0x2B);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardSpacebar")]
    pub const KeyboardSpacebar: Self = Self(0x2C);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardHyphen")]
    pub const KeyboardHyphen: Self = Self(0x2D);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardEqualSign")]
    pub const KeyboardEqualSign: Self = Self(0x2E);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardOpenBracket")]
    pub const KeyboardOpenBracket: Self = Self(0x2F);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardCloseBracket")]
    pub const KeyboardCloseBracket: Self = Self(0x30);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardBackslash")]
    pub const KeyboardBackslash: Self = Self(0x31);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardNonUSPound")]
    pub const KeyboardNonUSPound: Self = Self(0x32);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardSemicolon")]
    pub const KeyboardSemicolon: Self = Self(0x33);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardQuote")]
    pub const KeyboardQuote: Self = Self(0x34);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardGraveAccentAndTilde")]
    pub const KeyboardGraveAccentAndTilde: Self = Self(0x35);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardComma")]
    pub const KeyboardComma: Self = Self(0x36);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardPeriod")]
    pub const KeyboardPeriod: Self = Self(0x37);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardSlash")]
    pub const KeyboardSlash: Self = Self(0x38);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardCapsLock")]
    pub const KeyboardCapsLock: Self = Self(0x39);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF1")]
    pub const KeyboardF1: Self = Self(0x3A);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF2")]
    pub const KeyboardF2: Self = Self(0x3B);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF3")]
    pub const KeyboardF3: Self = Self(0x3C);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF4")]
    pub const KeyboardF4: Self = Self(0x3D);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF5")]
    pub const KeyboardF5: Self = Self(0x3E);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF6")]
    pub const KeyboardF6: Self = Self(0x3F);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF7")]
    pub const KeyboardF7: Self = Self(0x40);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF8")]
    pub const KeyboardF8: Self = Self(0x41);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF9")]
    pub const KeyboardF9: Self = Self(0x42);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF10")]
    pub const KeyboardF10: Self = Self(0x43);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF11")]
    pub const KeyboardF11: Self = Self(0x44);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF12")]
    pub const KeyboardF12: Self = Self(0x45);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardPrintScreen")]
    pub const KeyboardPrintScreen: Self = Self(0x46);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardScrollLock")]
    pub const KeyboardScrollLock: Self = Self(0x47);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardPause")]
    pub const KeyboardPause: Self = Self(0x48);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardInsert")]
    pub const KeyboardInsert: Self = Self(0x49);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardHome")]
    pub const KeyboardHome: Self = Self(0x4A);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardPageUp")]
    pub const KeyboardPageUp: Self = Self(0x4B);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardDeleteForward")]
    pub const KeyboardDeleteForward: Self = Self(0x4C);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardEnd")]
    pub const KeyboardEnd: Self = Self(0x4D);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardPageDown")]
    pub const KeyboardPageDown: Self = Self(0x4E);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardRightArrow")]
    pub const KeyboardRightArrow: Self = Self(0x4F);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLeftArrow")]
    pub const KeyboardLeftArrow: Self = Self(0x50);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardDownArrow")]
    pub const KeyboardDownArrow: Self = Self(0x51);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardUpArrow")]
    pub const KeyboardUpArrow: Self = Self(0x52);
    #[doc(alias = "UIKeyboardHIDUsageKeypadNumLock")]
    pub const KeypadNumLock: Self = Self(0x53);
    #[doc(alias = "UIKeyboardHIDUsageKeypadSlash")]
    pub const KeypadSlash: Self = Self(0x54);
    #[doc(alias = "UIKeyboardHIDUsageKeypadAsterisk")]
    pub const KeypadAsterisk: Self = Self(0x55);
    #[doc(alias = "UIKeyboardHIDUsageKeypadHyphen")]
    pub const KeypadHyphen: Self = Self(0x56);
    #[doc(alias = "UIKeyboardHIDUsageKeypadPlus")]
    pub const KeypadPlus: Self = Self(0x57);
    #[doc(alias = "UIKeyboardHIDUsageKeypadEnter")]
    pub const KeypadEnter: Self = Self(0x58);
    #[doc(alias = "UIKeyboardHIDUsageKeypad1")]
    pub const Keypad1: Self = Self(0x59);
    #[doc(alias = "UIKeyboardHIDUsageKeypad2")]
    pub const Keypad2: Self = Self(0x5A);
    #[doc(alias = "UIKeyboardHIDUsageKeypad3")]
    pub const Keypad3: Self = Self(0x5B);
    #[doc(alias = "UIKeyboardHIDUsageKeypad4")]
    pub const Keypad4: Self = Self(0x5C);
    #[doc(alias = "UIKeyboardHIDUsageKeypad5")]
    pub const Keypad5: Self = Self(0x5D);
    #[doc(alias = "UIKeyboardHIDUsageKeypad6")]
    pub const Keypad6: Self = Self(0x5E);
    #[doc(alias = "UIKeyboardHIDUsageKeypad7")]
    pub const Keypad7: Self = Self(0x5F);
    #[doc(alias = "UIKeyboardHIDUsageKeypad8")]
    pub const Keypad8: Self = Self(0x60);
    #[doc(alias = "UIKeyboardHIDUsageKeypad9")]
    pub const Keypad9: Self = Self(0x61);
    #[doc(alias = "UIKeyboardHIDUsageKeypad0")]
    pub const Keypad0: Self = Self(0x62);
    #[doc(alias = "UIKeyboardHIDUsageKeypadPeriod")]
    pub const KeypadPeriod: Self = Self(0x63);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardNonUSBackslash")]
    pub const KeyboardNonUSBackslash: Self = Self(0x64);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardApplication")]
    pub const KeyboardApplication: Self = Self(0x65);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardPower")]
    pub const KeyboardPower: Self = Self(0x66);
    #[doc(alias = "UIKeyboardHIDUsageKeypadEqualSign")]
    pub const KeypadEqualSign: Self = Self(0x67);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF13")]
    pub const KeyboardF13: Self = Self(0x68);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF14")]
    pub const KeyboardF14: Self = Self(0x69);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF15")]
    pub const KeyboardF15: Self = Self(0x6A);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF16")]
    pub const KeyboardF16: Self = Self(0x6B);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF17")]
    pub const KeyboardF17: Self = Self(0x6C);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF18")]
    pub const KeyboardF18: Self = Self(0x6D);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF19")]
    pub const KeyboardF19: Self = Self(0x6E);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF20")]
    pub const KeyboardF20: Self = Self(0x6F);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF21")]
    pub const KeyboardF21: Self = Self(0x70);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF22")]
    pub const KeyboardF22: Self = Self(0x71);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF23")]
    pub const KeyboardF23: Self = Self(0x72);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardF24")]
    pub const KeyboardF24: Self = Self(0x73);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardExecute")]
    pub const KeyboardExecute: Self = Self(0x74);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardHelp")]
    pub const KeyboardHelp: Self = Self(0x75);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardMenu")]
    pub const KeyboardMenu: Self = Self(0x76);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardSelect")]
    pub const KeyboardSelect: Self = Self(0x77);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardStop")]
    pub const KeyboardStop: Self = Self(0x78);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardAgain")]
    pub const KeyboardAgain: Self = Self(0x79);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardUndo")]
    pub const KeyboardUndo: Self = Self(0x7A);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardCut")]
    pub const KeyboardCut: Self = Self(0x7B);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardCopy")]
    pub const KeyboardCopy: Self = Self(0x7C);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardPaste")]
    pub const KeyboardPaste: Self = Self(0x7D);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardFind")]
    pub const KeyboardFind: Self = Self(0x7E);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardMute")]
    pub const KeyboardMute: Self = Self(0x7F);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardVolumeUp")]
    pub const KeyboardVolumeUp: Self = Self(0x80);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardVolumeDown")]
    pub const KeyboardVolumeDown: Self = Self(0x81);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLockingCapsLock")]
    pub const KeyboardLockingCapsLock: Self = Self(0x82);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLockingNumLock")]
    pub const KeyboardLockingNumLock: Self = Self(0x83);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLockingScrollLock")]
    pub const KeyboardLockingScrollLock: Self = Self(0x84);
    #[doc(alias = "UIKeyboardHIDUsageKeypadComma")]
    pub const KeypadComma: Self = Self(0x85);
    #[doc(alias = "UIKeyboardHIDUsageKeypadEqualSignAS400")]
    pub const KeypadEqualSignAS400: Self = Self(0x86);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardInternational1")]
    pub const KeyboardInternational1: Self = Self(0x87);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardInternational2")]
    pub const KeyboardInternational2: Self = Self(0x88);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardInternational3")]
    pub const KeyboardInternational3: Self = Self(0x89);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardInternational4")]
    pub const KeyboardInternational4: Self = Self(0x8A);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardInternational5")]
    pub const KeyboardInternational5: Self = Self(0x8B);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardInternational6")]
    pub const KeyboardInternational6: Self = Self(0x8C);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardInternational7")]
    pub const KeyboardInternational7: Self = Self(0x8D);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardInternational8")]
    pub const KeyboardInternational8: Self = Self(0x8E);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardInternational9")]
    pub const KeyboardInternational9: Self = Self(0x8F);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLANG1")]
    pub const KeyboardLANG1: Self = Self(0x90);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLANG2")]
    pub const KeyboardLANG2: Self = Self(0x91);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLANG3")]
    pub const KeyboardLANG3: Self = Self(0x92);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLANG4")]
    pub const KeyboardLANG4: Self = Self(0x93);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLANG5")]
    pub const KeyboardLANG5: Self = Self(0x94);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLANG6")]
    pub const KeyboardLANG6: Self = Self(0x95);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLANG7")]
    pub const KeyboardLANG7: Self = Self(0x96);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLANG8")]
    pub const KeyboardLANG8: Self = Self(0x97);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLANG9")]
    pub const KeyboardLANG9: Self = Self(0x98);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardAlternateErase")]
    pub const KeyboardAlternateErase: Self = Self(0x99);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardSysReqOrAttention")]
    pub const KeyboardSysReqOrAttention: Self = Self(0x9A);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardCancel")]
    pub const KeyboardCancel: Self = Self(0x9B);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardClear")]
    pub const KeyboardClear: Self = Self(0x9C);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardPrior")]
    pub const KeyboardPrior: Self = Self(0x9D);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardReturn")]
    pub const KeyboardReturn: Self = Self(0x9E);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardSeparator")]
    pub const KeyboardSeparator: Self = Self(0x9F);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardOut")]
    pub const KeyboardOut: Self = Self(0xA0);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardOper")]
    pub const KeyboardOper: Self = Self(0xA1);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardClearOrAgain")]
    pub const KeyboardClearOrAgain: Self = Self(0xA2);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardCrSelOrProps")]
    pub const KeyboardCrSelOrProps: Self = Self(0xA3);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardExSel")]
    pub const KeyboardExSel: Self = Self(0xA4);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLeftControl")]
    pub const KeyboardLeftControl: Self = Self(0xE0);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLeftShift")]
    pub const KeyboardLeftShift: Self = Self(0xE1);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLeftAlt")]
    pub const KeyboardLeftAlt: Self = Self(0xE2);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardLeftGUI")]
    pub const KeyboardLeftGUI: Self = Self(0xE3);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardRightControl")]
    pub const KeyboardRightControl: Self = Self(0xE4);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardRightShift")]
    pub const KeyboardRightShift: Self = Self(0xE5);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardRightAlt")]
    pub const KeyboardRightAlt: Self = Self(0xE6);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardRightGUI")]
    pub const KeyboardRightGUI: Self = Self(0xE7);
    #[doc(alias = "UIKeyboardHIDUsageKeyboard_Reserved")]
    pub const Keyboard_Reserved: Self = Self(0xFFFF);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardHangul")]
    pub const KeyboardHangul: Self = Self(UIKeyboardHIDUsage::KeyboardLANG1.0);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardHanja")]
    pub const KeyboardHanja: Self = Self(UIKeyboardHIDUsage::KeyboardLANG2.0);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardKanaSwitch")]
    pub const KeyboardKanaSwitch: Self = Self(UIKeyboardHIDUsage::KeyboardLANG1.0);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardAlphanumericSwitch")]
    pub const KeyboardAlphanumericSwitch: Self = Self(UIKeyboardHIDUsage::KeyboardLANG2.0);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardKatakana")]
    pub const KeyboardKatakana: Self = Self(UIKeyboardHIDUsage::KeyboardLANG3.0);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardHiragana")]
    pub const KeyboardHiragana: Self = Self(UIKeyboardHIDUsage::KeyboardLANG4.0);
    #[doc(alias = "UIKeyboardHIDUsageKeyboardZenkakuHankakuKanji")]
    pub const KeyboardZenkakuHankakuKanji: Self = Self(UIKeyboardHIDUsage::KeyboardLANG5.0);
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl Encode for UIKeyboardHIDUsage {
    const ENCODING: Encoding = CFIndex::ENCODING;
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl RefEncode for UIKeyboardHIDUsage {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
