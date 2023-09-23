def remove_vowels(s: str) -> str:
    final_s = ""
    for i in s:
        if i not in ('a', 'e', 'i', 'o', 'u'):
            final_s += i
    return final_s