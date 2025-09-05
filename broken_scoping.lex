var a = "global";
{
    fun showA() {
        print a;
    }
    showA();
    var a = "block";
    showA();
}

// expected behavior: global global
// actual behavior (prior to fixes): global block
