pub fn main() {
    // 1. Is there a workspace file?
    //    - Find the current Thor version
    //    - Check if that version of Thor exists
    //        - If not, download and install it
    //    - Call that version of Thor, passing through all arguments
    // 2. Is there command init?
    //    - Check CLI config file for latest installed version of Thor
    //        - If config file does not exist, call version API for latest version of Thor and download. Create config file.
    //    - Call that version of Thor, passing through all arguments
    // 3. Inform them to call `buri init` to initialize a workspace.
    println!("I am the CLI!");
}
