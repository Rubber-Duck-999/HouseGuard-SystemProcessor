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
        stage('Coverage') {
          steps {
              sh "cargo install cargo-tarpaulin"
              sh "cargo tarpaulin -v "
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
        stage("Deploy scripts") {
            steps {
                sh "cp -R deploy/ /home/simon"
            }
        }
    }
    post {
        failure {
            emailext body: 'Failed to build SYP', subject: 'Build Failure', to: '$DEFAULT_RECIPIENTS'
        }
    }
}
