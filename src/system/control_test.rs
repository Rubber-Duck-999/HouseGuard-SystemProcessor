use crate::system::constants;
use crate::Control;

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
        let mut controller = Control::new();
        let mut result = constants::FAULT_HANDLER.to_string();
        let mut component = controller.add_components_control(&mut result, true);
        let found:u8 = controller.exists_in_map(&result);
        assert_eq!(found, 1);
    }

    #[test]
    fn test_add_component_shutdown() {
        let mut controller = Control::new();
        let mut result = constants::FAULT_HANDLER.to_string();
        let mut component = controller.add_components_shutdown(&mut result);
        let found:u8 = controller.exists_in_map(&result);
        assert_eq!(found, 1);
    }   

}
