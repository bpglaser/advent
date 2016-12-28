#include <stdio.h>
#include <string.h>

int is_aba(char *s) {
    int b = s[0] != s[1] && s[0] == s[2] && s[0] != '[' && s[1] != ']' && s[1] != '[' && s[1] != ']';
    if (b) {
        for (size_t i = 0; i < 3; i++) {
            printf("%c", s[i]);
        }
        printf("\n");
    }
    return b;
}

int contains_bab(char *s, char a, char b) {
    int bracket_depth = 0;
    for (int i = 0; i < strlen(s) - 4; i++) {
        if (s[i] == '[') {
            bracket_depth++;
            continue;
        } else if (s[i] == ']') {
            bracket_depth--;
            continue;
        }

        printf("%c", s[i]);
        if (bracket_depth > 0 && s[i] == b && s[i + 1] == a && s[i + 2] == b) {
            printf("|");
            printf("%s\n", &s[i]);
            return 1;
        }
    }
    return 0;
}

int supports_ssl(char *line) {
    int bracket_depth = 0;
    for (int i = 0; i < strlen(line) - 4; i++) {
        if (line[i] == '[') {
            bracket_depth++;
            continue;
        } else if (line[i] == ']') {
            bracket_depth--;
            continue;
        }

        if (bracket_depth == 0 && is_aba(&line[i]) && contains_bab(line, line[i], line[i + 1])) {
            return 1;
        }
    }
    return 0;
}

int main(int argc, char const *argv[]) {
    char const *filename = argv[1];

    FILE *fp;
    char line[1024];

    fp = fopen(filename, "r");
    if (fp == NULL) {
        printf("Error opening file: %s\n", filename);
        return 1;
    }

    int count = 0;
    while (fgets(line, 1024, fp) != NULL) {
        if (supports_ssl(line)) {
            count++;
        }
    }
    printf("count: %i\n", count);

    return fclose(fp);
}
