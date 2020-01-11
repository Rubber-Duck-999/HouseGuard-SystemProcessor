use crate::Control;
use crate::system::constants;

#[cfg(test)]
mod tests 
{
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    #[should_panic]
    fn test_check_not_exists()
    {
        let mut controller = Control::new();
        let mut result = "---".to_string();
        let mut valid = controller.switch_names(&mut result);
        assert!(valid);
    }

    #[test]
    fn test_check_exists()
    {
        let mut controller = Control::new();
        let mut result = constants::FAULT_HANDLER.to_string();
        let mut valid = controller.switch_names(&mut result);
        assert!(valid);
    }

    #[test]
    fn test_check_switch()
    {
        let mut controller = Control::new();
        let mut result = constants::FAULT_HANDLER.to_string();
        let mut component = controller.switch_names(&mut result);
        assert_eq!(result, constants::FH_EXE);
    }
}