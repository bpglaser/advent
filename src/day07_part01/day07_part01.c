#include <stdio.h>
#include <string.h>

int is_abba(char *line) {
    int b = line[0] != line[1] && line[1] == line[2] && line[0] == line[3];
    if (b) {
        for (size_t i = 0; i < 4; i++) {
            printf("%c", line[i]);
        }
        printf("\n");
    }
    return line[0] != line[1] && line[1] == line[2] && line[0] == line[3];
}

void supports_tls(char *line, int *count) {
    int valid = 0;
    int bracket_depth = 0;
    for (int i = 0; i < strlen(line) - 5; i++) {
        if (line[i] == '[') {
            bracket_depth++;
            continue;
        } else if (line[i] == ']') {
            bracket_depth--;
            continue;
        }

        if (is_abba(&line[i])) {
            if (bracket_depth == 0) {
                valid++;
            } else if (bracket_depth > 0) {
                valid = 0;
                break;
            }
        }
    }
    if (valid) {
        (*count)++;
    }
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
        supports_tls(line, &count);
        // break;
    }
    printf("count: %i\n", count);

    return fclose(fp);
}
