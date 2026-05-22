/*
 * The ``if let`` syntax help you handle values that match one pattern while ignoring the rest
 * (in a less verbose way)
 *
 * ``if let PATTERN = VARIABLE {}``
 *
 *  *******************************************************
 *
 * The ``if let else`` syntax is used when you want to do something with other cases.
 *
 * ``if let PATTERN = VARIABLE {} else {}``
 *
 * ********************************************************
 *
 * Since ``if let else`` syntax can return an expression,
 * we can use ``let variable = if let ....`` to hold that expression.
 */

 #[allow(dead_code)]
 enum Setting {
     High(f32), // f32 represent temperatures
     Medium(f32),
     Low(f32)
 }

 impl Setting {
     fn display(&self) -> &'static str {
         match &self {
             Setting::High(_) => "high", // ``_`` placeholder indicates that all the temperatures result in the same outcome
             Setting::Medium(_) => "medium",
             Setting::Low(_) => "low",
         }
     }
 }

 fn main() {
     println!();

     // ----------------------------------------------------------------------------- //
     // ---------------------------- Without ``if let else`` ------------------------ //
     // ----------------------------------------------------------------------------- //

     let config_max = Some(3u8);
     match config_max {
         Some(config) => println!("The current configuration is: {config}"),
         _ => () // Do nothing
     }

     /*
      * ``Some(config)`` is the pattern arm, which is used to match ``Some(3u8)`` in ``config_max``.
      * If it does match, then variable ``config`` will take the value ``3u8``,
      * then print it out.
      *
      * We ignore all the remaining cases.
      */

      let setting_high = Setting::High(85.9);
      let mut other_count = 0;
      match setting_high {
           Setting::High(_f32) => println!("The current setting is {}", setting_high.display()),
          _ => other_count += 1
      }
      println!("other_count = {other_count}");

      /*
       * In this case, when the match encounters all the remaining cases other than ``Some(setting)``,
       * we increase the ``fail_count`` by 1.
       */

      println!("\n==============================================================\n");

     // ------------------------------------------------------------------------------------- //
     // ------------------------------- Using ``if let else`` ------------------------------- //
     // ------------------------------------------------------------------------------------- //

     // ``if let PATTERN = VARIABLE {}``
     let config_medium = Some(2u8);
     if let Some(config) = config_medium { // if let Some(config) = Some(2u8)
         println!("The current configuration is: {config}")
     }
     /*
      * With ``if let``, the same thing happens here: variable ``config`` will thatke the ``2u8`` value,
      * then we pring it out, but less codes, less typing, less indentation, less boilerplate.
      */

      ///////////////////////////////////////////////////////////////////////////////

     // ``if let PATTERN = VARIABLE {} else {}``
     let setting = Setting::Medium(70.3);
     let mut other_count = 0;
     if let Setting::Medium(_f32) = setting { // if let Setting::Medium = Setting::High
         println!("The current setting is {}", setting.display())
     } else {
         other_count += 1;
     }
     println!("other_count = {other_count}");
     /*
      * With ``if let else``,
      * if the pattern and the variable matched (Setting::Medium = Setting::Medium),
      * we print out that Medium setting.
      *
      * In other cases, ``else`` block will increase ``other_count`` by 1.
      */

      println!("\n==============================================================\n");

      // --------------------------------------------------------------------------------------------- //
      // ------------------------------- ``let variable = if let ...`` ------------------------------- //
      // --------------------------------------------------------------------------------------------- //

      fn temperature_check(setting: Setting) -> Option<String> {
          let temperature = if let Setting::High(temp) = setting { // Only check for ``Setting::High`` cases
              temp
          } else {
              return None // Handle cases where ``setting`` is not ``Setting::High``
          };

          if temperature.ge(&60.) & temperature.lt(&85.) {
              Some(format!("{temperature} celcius degree is safe")) // Output to be returned
          } else {
              Some(format!("{temperature} celcius degree is dangerous!!!"))
          }
      }

      let check = temperature_check(Setting::High(120.));
      match check {
          Some(message) => println!("{}", message),
          None => println!("We only check for ``Setting::High``")
      }
 }
