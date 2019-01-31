include!(concat!(env!("OUT_DIR"), "/types.rs"));

pub fn message_name(key: &u16) -> Option<&'static str> {
    type_value("mesg_num", &(*key as u32))
}
pub fn type_value(name: &str, key: &u32) -> Option<&'static str> {
    match name {
        "file" => file(key),
        "mesg_num" => mesg_num(key),
        "checksum" => checksum(key),
        "file_flags" => file_flags(key),
        "mesg_count" => mesg_count(key),
        "date_time" => date_time(key),
        "local_date_time" => local_date_time(key),
        "message_index" => message_index(key),
        "device_index" => device_index(key),
        "gender" => gender(key),
        "language" => language(key),
        "language_bits_0" => language_bits_0(key),
        "language_bits_1" => language_bits_1(key),
        "language_bits_2" => language_bits_2(key),
        "language_bits_3" => language_bits_3(key),
        "language_bits_4" => language_bits_4(key),
        "time_zone" => time_zone(key),
        "display_measure" => display_measure(key),
        "display_heart" => display_heart(key),
        "display_power" => display_power(key),
        "display_position" => display_position(key),
        "switch" => switch(key),
        "sport" => sport(key),
        "sport_bits_0" => sport_bits_0(key),
        "sport_bits_1" => sport_bits_1(key),
        "sport_bits_2" => sport_bits_2(key),
        "sport_bits_3" => sport_bits_3(key),
        "sport_bits_4" => sport_bits_4(key),
        "sport_bits_5" => sport_bits_5(key),
        "sport_bits_6" => sport_bits_6(key),
        "sub_sport" => sub_sport(key),
        "sport_event" => sport_event(key),
        "activity" => activity(key),
        "intensity" => intensity(key),
        "session_trigger" => session_trigger(key),
        "autolap_trigger" => autolap_trigger(key),
        "lap_trigger" => lap_trigger(key),
        "time_mode" => time_mode(key),
        "backlight_mode" => backlight_mode(key),
        "date_mode" => date_mode(key),
        "backlight_timeout" => backlight_timeout(key),
        "event" => event(key),
        "event_type" => event_type(key),
        "timer_trigger" => timer_trigger(key),
        "fitness_equipment_state" => fitness_equipment_state(key),
        "tone" => tone(key),
        "autoscroll" => autoscroll(key),
        "activity_class" => activity_class(key),
        "hr_zone_calc" => hr_zone_calc(key),
        "pwr_zone_calc" => pwr_zone_calc(key),
        "wkt_step_duration" => wkt_step_duration(key),
        "wkt_step_target" => wkt_step_target(key),
        "goal" => goal(key),
        "goal_recurrence" => goal_recurrence(key),
        "goal_source" => goal_source(key),
        "schedule" => schedule(key),
        "course_point" => course_point(key),
        "manufacturer" => manufacturer(key),
        "garmin_product" => garmin_product(key),
        "antplus_device_type" => antplus_device_type(key),
        "ant_network" => ant_network(key),
        "workout_capabilities" => workout_capabilities(key),
        "battery_status" => battery_status(key),
        "hr_type" => hr_type(key),
        "course_capabilities" => course_capabilities(key),
        "weight" => weight(key),
        "workout_hr" => workout_hr(key),
        "workout_power" => workout_power(key),
        "bp_status" => bp_status(key),
        "user_local_id" => user_local_id(key),
        "swim_stroke" => swim_stroke(key),
        "activity_type" => activity_type(key),
        "activity_subtype" => activity_subtype(key),
        "activity_level" => activity_level(key),
        "side" => side(key),
        "left_right_balance" => left_right_balance(key),
        "left_right_balance_100" => left_right_balance_100(key),
        "length_type" => length_type(key),
        "day_of_week" => day_of_week(key),
        "connectivity_capabilities" => connectivity_capabilities(key),
        "weather_report" => weather_report(key),
        "weather_status" => weather_status(key),
        "weather_severity" => weather_severity(key),
        "weather_severe_type" => weather_severe_type(key),
        "time_into_day" => time_into_day(key),
        "localtime_into_day" => localtime_into_day(key),
        "stroke_type" => stroke_type(key),
        "body_location" => body_location(key),
        "segment_lap_status" => segment_lap_status(key),
        "segment_leaderboard_type" => segment_leaderboard_type(key),
        "segment_delete_status" => segment_delete_status(key),
        "segment_selection_type" => segment_selection_type(key),
        "source_type" => source_type(key),
        "local_device_type" => local_device_type(key),
        "display_orientation" => display_orientation(key),
        "workout_equipment" => workout_equipment(key),
        "watchface_mode" => watchface_mode(key),
        "digital_watchface_layout" => digital_watchface_layout(key),
        "analog_watchface_layout" => analog_watchface_layout(key),
        "rider_position_type" => rider_position_type(key),
        "power_phase_type" => power_phase_type(key),
        "camera_event_type" => camera_event_type(key),
        "sensor_type" => sensor_type(key),
        "bike_light_network_config_type" => bike_light_network_config_type(key),
        "comm_timeout_type" => comm_timeout_type(key),
        "camera_orientation_type" => camera_orientation_type(key),
        "attitude_stage" => attitude_stage(key),
        "attitude_validity" => attitude_validity(key),
        "auto_sync_frequency" => auto_sync_frequency(key),
        "exd_layout" => exd_layout(key),
        "exd_display_type" => exd_display_type(key),
        "exd_data_units" => exd_data_units(key),
        "exd_qualifiers" => exd_qualifiers(key),
        "exd_descriptors" => exd_descriptors(key),
        "auto_activity_detect" => auto_activity_detect(key),
        "supported_exd_screen_layouts" => supported_exd_screen_layouts(key),
        "fit_base_type" => fit_base_type(key),
        "turn_type" => turn_type(key),
        "bike_light_beam_angle_mode" => bike_light_beam_angle_mode(key),
        "fit_base_unit" => fit_base_unit(key),
        "set_type" => set_type(key),
        "exercise_category" => exercise_category(key),
        "bench_press_exercise_name" => bench_press_exercise_name(key),
        "calf_raise_exercise_name" => calf_raise_exercise_name(key),
        "cardio_exercise_name" => cardio_exercise_name(key),
        "carry_exercise_name" => carry_exercise_name(key),
        "chop_exercise_name" => chop_exercise_name(key),
        "core_exercise_name" => core_exercise_name(key),
        "crunch_exercise_name" => crunch_exercise_name(key),
        "curl_exercise_name" => curl_exercise_name(key),
        "deadlift_exercise_name" => deadlift_exercise_name(key),
        "flye_exercise_name" => flye_exercise_name(key),
        "hip_raise_exercise_name" => hip_raise_exercise_name(key),
        "hip_stability_exercise_name" => hip_stability_exercise_name(key),
        "hip_swing_exercise_name" => hip_swing_exercise_name(key),
        "hyperextension_exercise_name" => hyperextension_exercise_name(key),
        "lateral_raise_exercise_name" => lateral_raise_exercise_name(key),
        "leg_curl_exercise_name" => leg_curl_exercise_name(key),
        "leg_raise_exercise_name" => leg_raise_exercise_name(key),
        "lunge_exercise_name" => lunge_exercise_name(key),
        "olympic_lift_exercise_name" => olympic_lift_exercise_name(key),
        "plank_exercise_name" => plank_exercise_name(key),
        "plyo_exercise_name" => plyo_exercise_name(key),
        "pull_up_exercise_name" => pull_up_exercise_name(key),
        "push_up_exercise_name" => push_up_exercise_name(key),
        "row_exercise_name" => row_exercise_name(key),
        "shoulder_press_exercise_name" => shoulder_press_exercise_name(key),
        "shoulder_stability_exercise_name" => shoulder_stability_exercise_name(key),
        "shrug_exercise_name" => shrug_exercise_name(key),
        "sit_up_exercise_name" => sit_up_exercise_name(key),
        "squat_exercise_name" => squat_exercise_name(key),
        "total_body_exercise_name" => total_body_exercise_name(key),
        "triceps_extension_exercise_name" => triceps_extension_exercise_name(key),
        "warm_up_exercise_name" => warm_up_exercise_name(key),
        "run_exercise_name" => run_exercise_name(key),
        "water_type" => water_type(key),
        "tissue_model_type" => tissue_model_type(key),
        "dive_gas_status" => dive_gas_status(key),
        "dive_alarm_type" => dive_alarm_type(key),
        "dive_backlight_mode" => dive_backlight_mode(key),
        "favero_product" => favero_product(key),
        _ => None,
    }
}
