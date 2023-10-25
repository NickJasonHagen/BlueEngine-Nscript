class devtools{
    func runcode(){
        if timerdiff(keytimer) < 99 {
            return
        }
        else {
            code = terminalinput("code?","exit")
            keytimer = timerinit()
            return code(code)   
        }

    }
    keytimer = timerinit()
}