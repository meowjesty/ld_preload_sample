#include <stdio.h>

void display_file(const char* filename) {
   char buffer[256];
   buffer[255] = 0;

   FILE* file = fopen(filename, "r");
   fgets(buffer, 256, (FILE*)file);

   printf("File contents:\n%s\n", buffer);

   fclose(file);
}

int main(int argc, char* argv[]) {
   display_file("./tmp/metalbear_somefile");
   display_file("./tmp/somefile");

   return 0;
}