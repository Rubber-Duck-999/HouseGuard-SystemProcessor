use crate::rabbitmq;
use crate::system::constants;
use crate::system::processes::Processes;
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
    fn test_check_switch_all() {
        let mut controller = Control::new();
        let mut result = "FH".to_string();
        let mut component = controller.switch_names(&mut result);
        assert_eq!(result, constants::FH_EXE);

        result = "NAC".to_string();
        component = controller.switch_names(&mut result);
        assert_eq!(result, constants::NAC_EXE);

        result = "CM".to_string();
        component = controller.switch_names(&mut result);
        assert_eq!(result, constants::CM_EXE);

        result = "EVM".to_string();
        component = controller.switch_names(&mut result);
        assert_eq!(result, constants::EVM_EXE);

        result = "DBM".to_string();
        component = controller.switch_names(&mut result);
        assert_eq!(result, constants::DBM_EXE);

        result = "UP".to_string();
        component = controller.switch_names(&mut result);
        assert_eq!(result, constants::UP_EXE);

        //Invalid name - so the name
        result = "Rabbitmq".to_string();
        component = controller.switch_names(&mut result);
        assert_eq!(result, "Rabbitmq".to_string());
        assert_eq!(component, false);
    }

    #[test]
    fn test_check_switch_shutdown() {
        let mut controller = Control::new();
        assert_eq!(controller.get_shutdown(), false);
        let mut result = "SYP".to_string();
        let mut component = controller.switch_names(&mut result);
        assert_eq!(result, "SYP".to_string());
        assert_eq!(controller.get_shutdown(), true);
    }

    #[test]
    fn test_add_component() {
        simple_logger::init_with_level(Level::Info).unwrap();
        let mut controller = Control::new();
        let mut result = constants::FH_EXE.to_string();
        controller.add_components_control(&mut result, true);
        let found: u8 = controller.exists_in_map(&result);
        assert_eq!(found, 1);
    }

    #[test]
    fn test_add_component_shutdown() {
        let mut controller = Control::new();
        let mut result = constants::FH_EXE.to_string();
        controller.add_components_shutdown(&mut result);
        let found: u8 = controller.exists_in_map(&result);
        assert_eq!(found, 0);
    }

    #[test]
    fn test_add_component_rubbish() {
        let mut controller = Control::new();
        let mut result = "crap".to_string();
        controller.add_components_control(&mut result, false);
        let found: u8 = controller.exists_in_map(&result);
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

    #[test]
    fn test_time_contains() {
        let mut controller = Control::new();
        let time = controller.get_time();
        if !time.contains("-") & !time.contains(":") {
            assert!(false);
        }
    }

    #[test]
    fn control_loop_exit() {
        let mut controller = Control::new();
        controller.set_shutdown();
        let time = controller.control_loop();
        assert_eq!(controller.get_event_counter(), 1);
    }

    #[test]
    fn request_test_valid() {
        let mut controller = Control::new();
        let mut message = rabbitmq::types::RequestPower {
            power: rabbitmq::types::SHUTDOWN.to_string(),
            severity: 1,
            component: "FH".to_string(),
        };
        controller.request_check(&mut message);
        assert_eq!(controller.get_event_counter(), 1);
    }

    #[test]
    fn request_test_invalid() {
        let mut controller = Control::new();
        let mut message = rabbitmq::types::RequestPower {
            power: rabbitmq::types::SHUTDOWN.to_string(),
            severity: 6,
            component: "FAH".to_string(),
        };
        controller.request_check(&mut message);
        assert_eq!(controller.get_event_counter(), 1);
    }

    #[test]
    fn test_start_no_file() {
        let mut controller = Control::new();
        let exists = controller.start("runFAH.sh", false);
        assert_eq!(exists, false);
        assert_eq!(controller.get_event_counter(), 1);
    }

    #[test]
    fn test_start_file_found() {
        let mut controller = Control::new();
        let exists = controller.start("runFH.sh", false);
        assert_eq!(exists, true);
    }

    #[test]
    fn test_start_file_found_and_kill() {
        let mut controller = Control::new();
        let mut result = "runFH.sh".to_string();
        let exists = controller.start(&result, true);
        assert_eq!(exists, true);

        let mut process_check = Processes::new();
        let mut test_runner_sh = constants::DEPLOY_SCRIPTS.to_owned() + &result.to_owned();
        let mut found = process_check.ps_find(&test_runner_sh);
        assert_eq!(found, 1);
        process_check.kill_main_component(&test_runner_sh);
        found = process_check.ps_find(&test_runner_sh);
        assert_eq!(found, 0);
    }

    #[test]
    fn request_check_no_processes() {
        let mut controller = Control::new();
        controller.check_process();
        assert_eq!(controller.get_event_counter(), 0);
    }

    /*#[test]
    #[should_panic]
    fn no_messages_consume() {
        let mut controller = Control::new();
        assert!(controller.consume_get());
    }*/
}
