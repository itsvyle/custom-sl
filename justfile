_default:
    just --list

bundle:
    rm -f custom-sl.py
    touch custom-sl.py
    echo "#!/usr/bin/env python3" >> custom-sl.py
    echo "" >> custom-sl.py
    \cat alphabet.py alphabet5.py main.py >> custom-sl.py 
    chmod +x custom-sl.py