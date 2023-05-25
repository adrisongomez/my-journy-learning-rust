pub mod people {
    pub trait Employee {
        fn get_job_id(&self) -> String;
        fn get_badge_number(&self) -> String;
        fn get_titular_name(&self) -> String;
    }

    pub trait Managable {
        fn get_team_id(&self) -> String;
    }

    #[derive(Debug)]
    pub struct Manager {
        pub job_id: String,
        pub badge_number: String,
        pub titular_name: String,
    }

    pub trait Supervisor {
        fn get_team_size(&self) -> u8;
    }

    pub trait Coder {
        fn get_language(&self) -> String;
        fn get_experince(&self) -> String;
    }


    #[derive(Debug)]
    pub struct Developer {
        pub job_id: String,
        pub badge_number: String,
        pub titular_name: String,
    }

    impl Supervisor for Manager {
        fn get_team_size(&self) -> u8 {
            10
        }
    }

    impl Employee for Manager {
        fn get_job_id(&self) -> String {
            self.job_id.clone()
        }

        fn get_badge_number(&self) -> String {
            self.badge_number.clone()
        }

        fn get_titular_name(&self) -> String {
            self.titular_name.clone()
        }
    }

    impl Managable for Manager {
        fn get_team_id(&self) -> String {
            "Team 1".to_string()
        }
    }

    impl Employee for Developer {
        fn get_job_id(&self) -> String {
            self.job_id.clone()
        }

        fn get_badge_number(&self) -> String {
            self.badge_number.clone()
        }

        fn get_titular_name(&self) -> String {
            self.titular_name.clone()
        }
    }

    impl Coder for Developer {

        fn get_language(&self) -> String {
            "Rust".to_string()
        }

        fn get_experince(&self) -> String {
            "2 years".to_string()
        }
    }
} 