terraform {
    required_providers {
        mongodbatlas = {
            source = "mongodb/mongodbatlas"
        }
    }
}

provider "mongodbatlas" {
    public_key = var.public_key
    private_key = var.private_key
}

resource "mongodbatlas_cluster" "poy_terraform" {
    name                        = "test-sinatra-db"
    project_id                  = "7d424a9c-2cb9-4b34-b307-c397e1461806"
    provider_instance_size_name = ""
    provider_name               = ""
}

