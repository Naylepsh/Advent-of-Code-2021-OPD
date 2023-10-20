module Lanternfish = struct
  let new_fish = 8
  let reset_fish = 6

  let rec tick fishes old_fishes new_fishes =
    match fishes with
    | [] -> List.rev old_fishes @ List.rev new_fishes
    | fish :: rest ->
        if fish == 0 then
          tick rest (reset_fish :: old_fishes) (new_fish :: new_fishes)
        else tick rest ((fish - 1) :: old_fishes) new_fishes

  let tick fishes = tick fishes [] []

  let rec tick_for n fishes =
    if n <= 0 then fishes else tick_for (n - 1) (tick fishes)

  let show fishes = String.concat "," (List.map string_of_int fishes)
  let from_string str = List.map int_of_string (String.split_on_char ',' str)
end

let read_lines file =
  In_channel.with_open_bin file In_channel.input_all
  |> String.split_on_char '\n'
  |> List.filter (fun line -> String.length line > 0)

let () =
  match read_lines "day6.input" with
  | line :: [] ->
      let fishes = Lanternfish.from_string line in
      let fs = Lanternfish.tick_for 80 fishes in
      print_endline (string_of_int (List.length fs))
  | _ -> print_endline "?"
