pipeline {
    agent any
    environment {
        REGISTRY = 'registry.aldev.tech'
        APPS = 'undangananas'
        REGISTRY_LOCATION = 'registry.aldev.tech/undangananas'
    }
    stages {
        stage('Build with docker') {
            steps {
                sh "docker build -t ${REGISTRY}/${APPS}:${BUILD_NUMBER} -t ${REGISTRY}/${APPS}:latest ."
            }
        }
        stage('Publish to docker registry') {
            parallel {
                stage('Publish current version') {
                    steps {
                        sh "docker push ${REGISTRY}/${APPS}:${BUILD_NUMBER}"
                    }
                }
                stage('Publish Latest') {
                    steps {
                        sh "docker push ${REGISTRY}/${APPS}:${BUILD_NUMBER}"
                    }
                }
            }
        }
    }
    post {
        always {
            echo "${REGISTRY_LOCATION}:${BUILD_NUMBER}"
        }
        success {
            echo 'I succeeded!!'
        }
        failure {
            echo 'I failed!'
        }
    }
}
