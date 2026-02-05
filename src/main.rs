/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/
use hello_remote_world::add;

fn main() {
    let name = "VS Code Remote - Containers";
    let c = add(1, 2);
    println!("Hello, {} with {}!", name, c);
}