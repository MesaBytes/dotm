end = '\33[0m'

class Color:
    def bold(self, msg): return '\33[1m' + msg + end
    def italic(self, msg): return '\33[3m' + msg + end
    def url(self, msg): return '\33[4m' + msg + end
    def blink(self, msg): return '\33[5m' + msg + end
    def blink2(self, msg): return '\33[6m' + msg + end
    def selected(self, msg): return '\33[7m' + msg + end
    def white(self, msg): return '\33[97m' + msg + end

    class dark:
        def black(self, msg): return '\33[30m' + msg + end
        def red(self, msg): return '\33[31m' + msg + end
        def green(self, msg): return '\33[32m' + msg + end
        def yellow(self, msg): return '\33[33m' + msg + end
        def blue(self, msg): return '\33[34m' + msg + end
        def violet(self, msg): return '\33[35m' + msg + end
        def beige(self, msg): return '\33[36m' + msg + end
        def white(self, msg): return '\33[37m' + msg + end

    class light:
        def grey(self, msg): return '\33[90m' + msg + end
        def red(self, msg): return '\33[91m' + msg + end
        def green(self, msg): return '\33[92m' + msg + end
        def yellow(self, msg): return '\33[93m' + msg + end
        def blue(self, msg): return '\33[94m' + msg + end
        def violet(self, msg): return '\33[95m' + msg + end
        def beige(self, msg): return '\33[96m' + msg + end

    class bg:
        def black(self, msg): return '\33[40m' + msg + end
        def red(self, msg): return '\33[41m' + msg + end
        def green(self, msg): return '\33[42m' + msg + end
        def yellow(self, msg): return '\33[43m' + msg + end
        def blue(self, msg): return '\33[44m' + msg + end
        def violet(self, msg): return '\33[45m' + msg + end
        def beige(self, msg): return '\33[46m' + msg + end
        def white(self, msg): return '\33[47m' + msg + end

        class light:
            def grey(self, msg): return '\33[100m' + msg + end
            def red(self, msg): return '\33[101m' + msg + end
            def green(self, msg): return '\33[102m' + msg + end
            def yellow(self, msg): return '\33[103m' + msg + end
            def blue(self, msg): return '\33[104m' + msg + end
            def violet(self, msg): return '\33[105m' + msg + end
            def beige(self, msg): return '\33[106m' + msg + end
            def white(self, msg): return '\33[107m' + msg + end

        light = light()
    bg = bg()
    dark = dark()
    light = light()

color = Color()