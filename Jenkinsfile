stage('Intro') {
    echo 'Greetings!'
}

node {
    stage('Build') {
        checkout scm

        sh 'make'
    }

    stage('Test') {
        sh 'make test'
    }
}
