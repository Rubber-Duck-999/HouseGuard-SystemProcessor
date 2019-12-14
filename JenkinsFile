pipeline {
    agent any
    stages {
        stage('Build') {
            steps {
                cargo build
            }
        }
        stage('Test') {
            steps {
                cargo test
            }
        }
        stage('Deploy') {
            steps {
                echo "Hello"
            }
        }
        stage("check code style") {
          steps {
              sh "./check-code-style.sh"
          }
        }
        stage("Integration") {
          steps {
            sh "./runIntegrationTest.sh"
          }
        }
        stage("Vulnerabilities Test") {
          steps {
            cargo install --force cargo-audit
            cargo generate-lockfile
            cargo audit
          }
        }
        stage("Lint") {
          steps {
            rustup component add clippy
            cargo clippy
          }
        }
    }
}
