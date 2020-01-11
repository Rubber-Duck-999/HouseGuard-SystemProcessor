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
              sh "cargo tarpulin -v "
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
        always {
            echo 'This will always run'
        }
        success {
            echo 'This will run only if successful'
        }
        failure {
            mail bcc: '', body: "<b>Example</b><br>\n\<br>Project: ${env.JOB_NAME} <br>Build Number: ${env.BUILD_NUMBER} <br> URL de build: ${env.BUILD_URL}", cc: '', charset: 'UTF-8', from: '', mimeType: 'text/html', replyTo: '', subject: "ERROR CI: Project name -> ${env.JOB_NAME}", to: '$DEFAULT_RECIPIENTS';
        }
        unstable {
            echo 'This will run only if the run was marked as unstable'
        }
        changed {
            echo 'This will run only if the state of the Pipeline has changed'
            echo 'For example, if the Pipeline was previously failing but is now successful'
        }
    }
}
