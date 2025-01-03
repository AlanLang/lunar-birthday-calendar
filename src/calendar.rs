use chinese_lunisolar_calendar::{LunarDay, LunarMonth, LunisolarDate, SolarDate};
use chrono::{Datelike, NaiveDate, Utc};
use icalendar::{Calendar, Component, Event, EventLike};

use crate::config;

pub fn create_calendar(config: config::BirthdayConfig) -> Calendar {
  // 生成 去年 今年 明年 三个年份的 vec 数据
  let years = vec![
    Utc::now().year() as u16 - 1,
    Utc::now().year() as u16,
    Utc::now().year() as u16 + 1,
  ];
  let mut calendar = Calendar::new();
  years.iter().for_each(|year| {
    config.birthdays.iter().for_each(|birthday| {
      let date = create_solar_birthday(birthday.clone(), *year);
      match date {
        Ok(result) => {
          let (date, month, day) = result;
          let event = Event::new()
            .summary(&format!(
              "🎂 {}的生日-农历{:#}{:#}",
              birthday.name, month, day
            ))
            .all_day(date)
            .done();
          calendar.push(event);
        }
        Err(err) => {
          log::error!("{}", err);
        }
      }
    });
  });
  calendar
}

fn create_solar_birthday(
  birthday: config::Birthday,
  year: u16,
) -> anyhow::Result<(NaiveDate, LunarMonth, LunarDay)> {
  let lunar =
    LunisolarDate::from_ymd(year, birthday.month, false, birthday.day).map_err(|err| {
      anyhow::anyhow!(
        "Failed to create lunar date for birthday '{}': {}",
        birthday.name,
        err
      )
    })?;
  let gregorian = SolarDate::from(lunar);
  let lunar_month = lunar.to_lunar_month();
  let lunar_day = lunar.to_lunar_day();
  let date = gregorian.to_naive_date();
  Ok((date, lunar_month, lunar_day))
}
