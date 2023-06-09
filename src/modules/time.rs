use psp::{sys::{ sceRtcGetCurrentClockLocalTime, ScePspDateTime, sceRtcGetTick, sceRtcFormatRFC3339, sceRtcConvertLocalTimeToUTC, sceRtcGetDayOfWeek }, dprintln};

pub unsafe fn base() {
    // Where actual date time is stored
    let record = &mut ScePspDateTime::default();

    // Assign time
    sceRtcGetCurrentClockLocalTime(record);
    
    let ScePspDateTime { year, month, day, hour, minutes, seconds, .. } = record;

    // Obtain day of week
    let day_week = sceRtcGetDayOfWeek(*year as i32, *month as i32, *day as i32);
    dprintln!("Actual date is: {day:}.{month:}.{year:} {hour:}:{minutes:}.{seconds:} {day_week}/7");
}
