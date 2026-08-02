use crate::{
    EffectStruct,
    colors::{INDICATOR_BLUE, INDICATOR_VIOLET},
};
use lib::{Effect, Effects};
use slint::VecModel;

pub fn effect_to_model(effect: Effect, id: i32) -> EffectStruct {
    let has_value = match effect {
        Effect::Invert | Effect::ReverseBits => false,
        _ => true,
    };

    let value = match effect {
        Effect::Brightness(val) | Effect::WrapBrightness(val) => Some(val as i32),
        Effect::Min(val) | Effect::Max(val) => Some(val as i32),
        Effect::Invert | Effect::ReverseBits => None,
    };

    let indicator_color = match effect {
        Effect::Brightness(_) | Effect::WrapBrightness(_) | Effect::Min(_) | Effect::Max(_) => {
            INDICATOR_BLUE
        }
        Effect::Invert | Effect::ReverseBits => INDICATOR_VIOLET,
    };

    let (min_value, max_value) = match effect {
        Effect::Brightness(_) | Effect::WrapBrightness(_) => (-255, 255),
        Effect::Min(_) | Effect::Max(_) => (0, 255),
        _ => (0, 0),
    };

    let indicator_color = indicator_color.into();
    let value = value.unwrap_or(0).into();

    EffectStruct {
        id,
        name: effect.to_type().into(),
        has_value,
        value,
        min_value,
        max_value,
        indicator_color,
    }
}

pub fn effects_to_model(effects: &Effects) -> slint::ModelRc<EffectStruct> {
    let items = effects
        .inner
        .iter()
        .cloned()
        .enumerate()
        .map(|(id, effect)| effect_to_model(effect, id as i32))
        .collect::<Vec<_>>();

    slint::ModelRc::new(VecModel::from(items))
}
