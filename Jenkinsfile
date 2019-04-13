stage('intro') {
    echo 'Greetings!'
}

node {
    stage('checkout') {
        checkout scm
    }

    stage('make') {
        sh 'make'
    }

    stage('test make') {
        sh 'make test'
    }

    stage('cmake') {
        sh '''
            mkdir build
            pushd build
            cmake ..
            popd
        '''
    }

    stage('test cmake') {
        sh 'build/loadavg'
    }
}
