
function quickrun
    set -l langs "c" "rust" "rust (backtrace)" "rust test" "haskell (no file)" "haskell (load file)" "dotter deploy" "custom"

    set -l chosen "unset"

    if test (count $argv) -eq  1
        if test $argv = "-r"
            set chosen (printf "%s\n" $langs | fzf)
            echo $chosen > .quickrun
        else
            set chosen (cat .quickrun)
        end
    else
        if test -e .quickrun
            set chosen (cat .quickrun)
        else
            set chosen (printf "%s\n" $langs | fzf)
            echo $chosen > .quickrun
        end
    end

    switch $chosen
        case c
            cr
        case "rust" "rust run"
            cargo run
        case "rust (backtrace)"
            begin; set -lx RUST_BACKTRACE 1; cargo run; end;
        case "rust test"
            begin; set -lx RUST_BACKTRACE 1; cargo test; end;
        case "haskell (no file)"
            ghci
        case "haskell (load file)"
            ghci *.hs
        case "dotter deploy"
            dotter -v
            if test $status = 0
                echo "deploy successful"
            else
                echo "deploy failed"
            end
        case "custom"
            cat .quickrun-exe
            eval (cat .quickrun-exe)
            if test $status = 1 # this is a dirty nasty hack, pls FIXME
                return
            end
        case "*"
            echo $chosen " not yet implemented"
    end
    read
end
