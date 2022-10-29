package main

import (
    "dagger.io/dagger"
    "dagger.io/dagger/core"
    "universe.dagger.io/bash"
    "universe.dagger.io/docker"
)

#Cargo: {
    commands: string

    checkoutCode: core.#Source & {
        path: "."
    }

    pullRustImage: docker.#Pull & {
        source: "rust:1-buster"
    }

    copyCodeToImage: docker.#Copy & {
        input: pullRustImage.output
        contents: checkoutCode.output
    }

    runCargo: bash.#Run & {
        input: copyCodeToImage.output
        script: contents: commands
        export: directories: "/target": _
    }
}

dagger.#Plan & {
    client: {
        filesystem: {
            "./target": write: contents: actions.build.runCargo.export.directories."/target"
        }
    }
    actions: {
        build: #Cargo & {
            commands: "cargo build --release"
        }
        test: #Cargo & {
            commands: "cargo test"
        }
    }
}