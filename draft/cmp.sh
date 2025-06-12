files=(./-*)
for ((i = 0; i < ${#files[@]}; i++)); do
  for ((j = i + 1; j < ${#files[@]}; j++)); do
    f1="${files[$i]}"
    f2="${files[$j]}"

    # Verifica se ambos os arquivos ainda existem
    [[ -e "$f1" ]] || continue
    [[ -e "$f2" ]] || continue

    # Armazena diff
    D=$(echo diff -u "$f1" "$f2")
    echo $D
    D=$(eval $D)

    if [[ -z "$D" ]]; then
      echo "--- arquivos idênticos: $f1 == $f2"
      rm -v "$f2"
    else
      echo "=== diff: $f1 <-> $f2 ==="
      echo "$D"
    fi
  done
done
