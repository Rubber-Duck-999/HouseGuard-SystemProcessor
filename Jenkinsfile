pipeline {
    agent any
    options {
        parallelsAlwaysFailFast()
    }
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
              sh "cargo tarpaulin -v -- --test-threads=1"
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

        stage('Parallel Stage') {
            when {
                branch 'master'
            }
            parallel {
                stage('Python') {
                    agent {
                        label "for-running-integrator"
                    }
                    steps {
                        echo "Python"
                    }
                }
                stage('Rust') {
                    agent {
                        label "for-running-tests"
                    }
                    steps {
                        echo "SYP"
                    }
                }
            }
        }
    }
    post {
        failure {
            emailext body: 'Failed to build SYP', subject: 'Build Failure', to: '$DEFAULT_RECIPIENTS'
        }
    }
}

