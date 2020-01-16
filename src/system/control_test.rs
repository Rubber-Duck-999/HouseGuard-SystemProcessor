use crate::system::constants;
use crate::Control;
use log::Level;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    #[should_panic]
    fn test_check_not_exists() {
        let mut controller = Control::new();
        let mut result = "---".to_string();
        let mut valid = controller.switch_names(&mut result);
        assert!(valid);
    }

    #[test]
    fn test_check_exists() {
        let mut controller = Control::new();
        let mut result = constants::FAULT_HANDLER.to_string();
        let mut valid = controller.switch_names(&mut result);
        assert!(valid);
    }

    #[test]
    fn test_check_switch() {
        let mut controller = Control::new();
        let mut result = constants::FAULT_HANDLER.to_string();
        let mut component = controller.switch_names(&mut result);
        assert_eq!(result, constants::FH_EXE);
    }

    #[test]
    fn test_add_component() {
        simple_logger::init_with_level(Level::Info).unwrap();
        let mut controller = Control::new();
        let mut result = constants::FH_EXE.to_string();
        controller.add_components_control(&mut result, true);
        let found:u8 = controller.exists_in_map(&result);
        assert_eq!(found, 1);
    }

    #[test]
    fn test_add_component_shutdown() {
        let mut controller = Control::new();
        let mut result = constants::FH_EXE.to_string();
        controller.add_components_shutdown(&mut result);
        let found:u8 = controller.exists_in_map(&result);
        assert_eq!(found, 0);
    }

    #[test]
    #[should_panic]
    fn test_get_time_none_letters() {
        let mut controller = Control::new();
        let time = controller.get_time();
        let result = time.chars().all(char::is_alphanumeric);
        assert!(result);
    }

    fn test_time_contains() {
        let mut controller = Control::new();
        let time = controller.get_time();
        if !time.contains("-") & !time.contains(":") {
            assert!(false);
        } 
    }
}
