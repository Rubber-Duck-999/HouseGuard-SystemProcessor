pipeline {
    agent any
    stages {
        stage('Build') {
            steps {
                sh "cargo build"
            }
        }
        stage('Test') {
            steps {
                sh "cargo test -- --test-threads=1"
            }
        }
        stage('Deploy') {
            steps {
                echo "Hello"
            }
        }
        stage("check code style") {
          steps {
              sh "chmod +x ./check-code-style.sh"
              sh "./check-code-style.sh"
          }
        }
        stage("Integration") {
          steps {
            sh "chmod +x ./runIntegrationTest.sh"
            sh "./runIntegrationTest.sh"
          }
        }
        stage("Vulnerabilities Test") {
          steps {
            sh "cargo install --force cargo-audit"
            sh "cargo generate-lockfile"
            sh "cargo audit"
          }
        }
    }
}
