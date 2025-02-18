
use agb::sound::dmg::{DutyCycle, EnvelopeSettings, SoundDirection, SweepSettings};

pub fn play_sound(track: u16, vblank: &agb::interrupt::VBlank, sound: &agb::sound::dmg::Sound) {
    //must wait, as on real hardware channel 1 wont work consistently otherwise
    vblank.wait_for_vblank();
    //menu move
    if track == 0 {
        let sweep_settings = SweepSettings::new(5, SoundDirection::Increase, 2);
        let envelope_settings = EnvelopeSettings::new(2, SoundDirection::Decrease, 3);
        let duty_cycle = DutyCycle::ThreeQuarters;

        let frequency = 1300;
        sound.channel1().play_sound(
            frequency,
            Some(0),
            &sweep_settings,
            &envelope_settings,
            duty_cycle,
        );
    }
    //switch
    else if track == 1 {
        let envelope_settings: EnvelopeSettings =
            EnvelopeSettings::new(2, SoundDirection::Decrease, 8);
        sound
            .noise()
            .play_sound(Some(0), &envelope_settings, 4, true, 3);
    }
    //jump
    else if track == 2 {
        let sweep_settings = SweepSettings::new(4, SoundDirection::Increase, 5);
        let envelope_settings = EnvelopeSettings::new(3, SoundDirection::Decrease, 10);
        let duty_cycle = DutyCycle::Half;
        let frequency = 1000;
        sound.channel1().play_sound(
            frequency,
            Some(0),
            &sweep_settings,
            &envelope_settings,
            duty_cycle,
        );
    }
    //land
    else if track == 3 {
        let envelope_settings: EnvelopeSettings =
            EnvelopeSettings::new(2, SoundDirection::Decrease, 3);
        sound
            .noise()
            .play_sound(Some(0), &envelope_settings, 5, true, 5); //2, true, 1
    }
    //cancel
    else if track == 4 {
        let envelope_settings: EnvelopeSettings =
            EnvelopeSettings::new(2, SoundDirection::Decrease, 12);
        sound
            .noise()
            .play_sound(Some(0), &envelope_settings, 5, true, 6);
    }
    //finish?
    else if track == 5 {
        let sweep_settings = SweepSettings::new(5, SoundDirection::Increase, 3);
        let envelope_settings = EnvelopeSettings::new(7, SoundDirection::Decrease, 9);
        let duty_cycle = DutyCycle::OneQuarter;
        let frequency = 1300;
        sound.channel1().play_sound(
            frequency,
            Some(0),
            &sweep_settings,
            &envelope_settings,
            duty_cycle,
        );
    }
    //menu ok
    else if track == 6 {
        let sweep_settings = SweepSettings::new(3, SoundDirection::Increase, 6);
        let envelope_settings = EnvelopeSettings::new(7, SoundDirection::Decrease, 9);
        let duty_cycle = DutyCycle::OneQuarter;
        let frequency = 1300;
        sound.channel1().play_sound(
            frequency,
            Some(0),
            &sweep_settings,
            &envelope_settings,
            duty_cycle,
        );
    }
}
