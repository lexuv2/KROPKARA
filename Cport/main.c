#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int SEED;

int hash[] = { 208, 34, 231, 213, 32, 248, 233, 56, 161, 78, 24, 140, 71, 48, 140, 254, 245, 255, 247, 247, 40,
    185, 248, 251, 245, 28, 124, 204, 204, 76, 36, 1, 107, 28, 234, 163, 202, 224, 245, 128, 167, 204,
    9, 92, 217, 54, 239, 174, 173, 102, 193, 189, 190, 121, 100, 108, 167, 44, 43, 77, 180, 204, 8, 81,
    70, 223, 11, 38, 24, 254, 210, 210, 177, 32, 81, 195, 243, 125, 8, 169, 112, 32, 97, 53, 195, 13,
    203, 9, 47, 104, 125, 117, 114, 124, 165, 203, 181, 235, 193, 206, 70, 180, 174, 0, 167, 181, 41,
    164, 30, 116, 127, 198, 245, 146, 87, 224, 149, 206, 57, 4, 192, 210, 65, 210, 129, 240, 178, 105,
    228, 108, 245, 148, 140, 40, 35, 195, 38, 58, 65, 207, 215, 253, 65, 85, 208, 76, 62, 3, 237, 55, 89,
    232, 50, 217, 64, 244, 157, 199, 121, 252, 90, 17, 212, 203, 149, 152, 140, 187, 234, 177, 73, 174,
    193, 100, 192, 143, 97, 53, 145, 135, 19, 103, 13, 90, 135, 151, 199, 91, 239, 247, 33, 39, 145,
    101, 120, 99, 3, 186, 86, 99, 41, 237, 203, 111, 79, 220, 135, 158, 42, 30, 154, 120, 67, 87, 167,
    135, 176, 183, 191, 253, 115, 184, 21, 233, 58, 129, 233, 142, 39, 128, 211, 118, 137, 139, 255,
    114, 20, 218, 113, 154, 27, 127, 246, 250, 1, 8, 198, 250, 209, 92, 222, 173, 21, 88, 102, 219 };

int noise2(int x, int y)
{
    int tmp = hash[(y + SEED) % 256];
    return hash[(tmp + x) % 256];
}

float lin_inter(float x, float y, float s)
{
    return x + s * (y - x);
}

float smooth_inter(float x, float y, float s)
{
    return lin_inter(x, y, s * s * (3 - 2 * s));
}

float noise2d(float x, float y)
{
    int x_int = x;
    int y_int = y;
    float x_frac = x - x_int;
    float y_frac = y - y_int;
    int s = noise2(x_int, y_int);
    int t = noise2(x_int + 1, y_int);
    int u = noise2(x_int, y_int + 1);
    int v = noise2(x_int + 1, y_int + 1);
    float low = smooth_inter(s, t, x_frac);
    float high = smooth_inter(u, v, x_frac);
    return smooth_inter(low, high, y_frac);
}

float value2d(float x, float y, float freq, int depth)
{
    float xa = x * freq;
    float ya = y * freq;
    float amp = 1.0;
    float fin = 0;
    float div = 0.0;

    int i;
    for (i = 0; i < depth; i++) {
        div += 256 * amp;
        fin += noise2d(xa, ya) * amp;
        amp /= 2;
        xa *= 2;
        ya *= 2;
    }

    return fin / div;
}

double start_height(int64_t x, int64_t y)
{
    double xd = x;
    double yd = y;
    double q = 0;
    double modif = 1.0;
    for (int i = (1 << 10); i; i = i >> 1) {
        //        printf("%d ", q)
        xd /= 2;
        yd /= 2;
        modif *= 1.5;
        q += value2d(xd, yd, .1, 1) * modif;
    }
    return q;
}

struct map {
    int64_t size_x;
    int64_t size_y;
    double** data;
};
typedef struct map map;

int64_t map_generate(map* self, int64_t x, int64_t y)
{
    self->data = (double**)malloc(x * sizeof(double*));
    if (self->data == NULL) {
        printf("failed to allocate x row");
        return 1;
    }
    for (int64_t i = 0; i < x; ++i) {
        self->data[i] = (double*)malloc(y * sizeof(double));
        if (self->data[i] == NULL) {
            printf("failed to allocate y row %d", i);
            for (int64_t q = 0; q < i; ++i) {
                free(self->data[q]);
            }
            free(self->data);
            return 2;
        }
        for (int64_t j = 0; j < y; ++j) {
            self->data[i][j] = start_height(i, j);
        }
    }
    self->size_x = x;
    self->size_y = y;
    return 0;
}

void map_regenerate(map* self)
{
    for (int64_t i = 0; i < self->size_x; ++i) {
        for (int64_t j = 0; j < self->size_y; ++j) {
            self->data[i][j] = start_height(i, j);
        }
    }
    return;
}

int64_t map_fill(map* self, double d, int64_t x, int64_t y)
{
    self->data = (double**)malloc(x * sizeof(double*));
    if (self->data == NULL) {
        printf("failed to allocate x row");
        return 1;
    }
    for (int64_t i = 0; i < x; ++i) {
        self->data[i] = (double*)malloc(y * sizeof(double));
        if (self->data[i] == NULL) {
            printf("failed to allocate y row %d", i);
            for (int64_t q = 0; q < i; ++i) {
                free(self->data[q]);
            }
            free(self->data);
            return 2;
        }
        for (int64_t j = 0; j < y; ++j) {
            self->data[i][j] = d;
        }
    }
    self->size_x = x;
    self->size_y = y;
    return 0;
}

void map_refill(map* self, double d)
{
    for (int64_t i = 0; i < self->size_x; ++i) {
        for (int64_t j = 0; j < self->size_y; ++j) {
            self->data[i][j] = d;
        }
    }
    return;
}

double map_sweep(map* self)
{
    double q = 0;
    for (int i = 0; i < self->size_x; ++i)
        for (int j = 0; j < self->size_y; ++j)
            q += self->data[i][j];
    return q;
}

int64_t map_save(map* self, char* filename)
{
    FILE* fptr;
    fptr = fopen(filename, "w");
    if (fptr == NULL)
        return 1;
    fprintf(fptr, "%d;%d\n", self->size_x, self->size_y);
    for (int64_t i = 0; i < self->size_x; ++i) {
        for (int64_t j = 0; j < self->size_y; ++j) {
            fprintf(fptr, "%lf;", self->data[i][j]);
        }
        fprintf(fptr, "\n");
    }
    fclose(fptr);
    return 0;
}

void map_copy(map* src, map* tg)
{
    tg->size_x = src->size_x;
    tg->size_y = src->size_y;
    for (int x = 0; x < src->size_x; ++x)
        for (int y = 0; y < src->size_y; ++y)
            tg->data[x][y] = src->data[x][y];
}

struct local_stack {
    int64_t middlex;
    int64_t middley;
    double dirt[3][3];
    double water[3][3];
};
typedef struct local_stack local_stack;

local_stack get_surroundings(map* water_levels, map* dirt_levels, int64_t x, int64_t y)
{
    local_stack local;
    local.middlex = x;
    local.middley = y;
    for (int xo = -1; xo < 2; ++xo) {
        for (int yo = -1; yo < 2; ++yo) {
            if ((x + xo >= 0) && (x + xo < water_levels->size_x) && (y + yo >= 0) && (y + yo < water_levels->size_y)) {
                local.dirt[xo + 1][yo + 1] = dirt_levels->data[x + xo][y + yo];
                local.water[xo + 1][yo + 1] = water_levels->data[x + xo][y + yo];
            } else if ((x + xo >= 0) && (x + xo < water_levels->size_x)) { // x ok y out of bounds
                if (y + yo < 0) {
                    local.dirt[xo + 1][yo + 1] = dirt_levels->data[x + xo][y + yo + 1];
                    local.water[xo + 1][yo + 1] = water_levels->data[x + xo][y + yo + 1];
                } else {
                    local.dirt[xo + 1][yo + 1] = dirt_levels->data[x + xo][y + yo - 1];
                    local.water[xo + 1][yo + 1] = water_levels->data[x + xo][y + yo - 1];
                }
            } else if ((y + yo >= 0) && (y + yo < water_levels->size_y)) { // y ok x out of bounds
                if (x + xo < 0) {
                    local.dirt[xo + 1][yo + 1] = dirt_levels->data[x + xo + 1][y + yo];
                    local.water[xo + 1][yo + 1] = water_levels->data[x + xo + 1][y + yo];
                } else {
                    local.dirt[xo + 1][yo + 1] = dirt_levels->data[x + xo - 1][y + yo];
                    local.water[xo + 1][yo + 1] = water_levels->data[x + xo - 1][y + yo];
                }
            } else {
                local.dirt[xo + 1][yo + 1] = dirt_levels->data[x][y];
                local.water[xo + 1][yo + 1] = water_levels->data[x][y];
            }
        }
    }

    return local;
}

void paste_surroundings(map* water_levels, map* dirt_levels, local_stack v)
{
    for (int xo = 0; xo < 3; ++xo) {
        for (int yo = 0; yo < 3; ++yo) {
            if ((v.middlex + xo - 1 >= 0) && (v.middlex + xo - 1 < water_levels->size_x) && (v.middley + yo - 1 >= 0) && (v.middley + yo - 1 < water_levels->size_y)) {
                water_levels->data[v.middlex + xo - 1][v.middley + yo - 1] += v.water[xo][yo];
                dirt_levels->data[v.middlex + xo - 1][v.middley + yo - 1] += v.dirt[xo][yo];
            }
        }
    }
}

double small_squash(double v)
{
    if (v * v < 0.0001)
        return 0;
    return v;
}

int64_t waterfall(map* self, double rain, double until, double seeping)
{
    map water[2];
    map_fill(water, rain, self->size_x, self->size_y);
    map_fill(water + 1, rain, self->size_x, self->size_y);
    map dirt[2];
    map_fill(dirt, 0, self->size_x, self->size_y);
    map_fill(dirt + 1, 0, self->size_x, self->size_y);
    map_copy(self, dirt);

    int64_t i = 0;
    double water_initial = map_sweep(water);
    double water_remaining = water_initial;
    double local_average = 0;
    double diff;
    double dirt_initial = map_sweep(dirt);
    double dirt_remaining;

    for (; water_remaining > water_initial * until; i ^= 1) {
        map_copy(dirt + i, dirt + (i ^ 1));
        map_copy(water + i, water + (i ^ 1));
        for (int x = 0; x < self->size_x; ++x)
            for (int y = 0; y < self->size_y; ++y)
                water[i].data[x][y] *= seeping;
        for (int x = 0; x < self->size_x; ++x) {
            for (int y = 0; y < self->size_y; ++y) {
                local_stack here = get_surroundings(water + i, dirt + i, x, y);
                local_average = 0;
                for (int xo = 0; xo < 3; ++xo)
                    for (int yo = 0; yo < 3; ++yo)
                        local_average += here.dirt[xo][yo] + here.water[xo][yo];
                local_average /= 9.0;
                if (here.dirt[1][1] + here.water[1][1] > local_average) {
                    local_stack change;
                    for (int xo = 0; xo < 3; ++xo)
                        for (int yo = 0; yo < 3; ++yo) {
                            change.water[xo][yo] = local_average - here.water[xo][yo] - here.dirt[xo][yo];
                            change.dirt[xo][yo] = (change.water[xo][yo] / (here.water[xo][yo] + change.water[xo][yo])) * (here.dirt[1][1] - here.dirt[xo][yo]);
                            if (!(xo == yo == 1)) {
                                if (change.water[xo][yo] < 0)
                                    change.water[xo][yo] = 0;
                                if (change.dirt[xo][yo] < 0)
                                    change.dirt[xo][yo] = 0;
                            }
                        }
                    double sum_outflow = 0;
                    double sum_dirtflow = 0;
                    for (int xo = 0; xo < 3; ++xo)
                        for (int yo = 0; yo < 3; ++yo) {
                            if (!(xo == yo == 1)) {
                                sum_outflow += change.water[xo][yo];
                                sum_dirtflow += change.dirt[xo][yo];
                            }
                        }
                    change.water[1][1] = -sum_outflow;
                    change.dirt[1][1] = -sum_dirtflow;
                    change.middlex = x;
                    change.middley = y;

                    paste_surroundings(water + (i ^ 1), dirt + (i ^ 1), change);
                } else {
                    // printf("not over local average with %lf + %lf against %lf\n", here.dirt[1][1], here.water[1][1], local_average);
                }
            }
        }
        water_remaining = map_sweep(water + (i ^ 1));
        dirt_remaining = map_sweep(dirt + (i ^ 1));
        if (dirt_remaining < 0) {
            printf("negative dirt remaining");
            map_save(dirt + (i ^ 1), "error.txt");
            return 1;
        }
        printf("water initial: %lf, water remaining: %lf   ", water_initial, water_remaining);
        printf("dirt initial: %lf, dirt remaining: %lf\n", dirt_initial, dirt_remaining);
    }
    map_copy(dirt + (i ^ 1), self);

    return 0;
}

int main()
{
    srand(time(NULL));
    SEED = rand();
    char t[256];
    for (int i = 0; i < 256; ++i)
        t[i] = 0;
    int64_t _;
    for (int i = 0; i < 256; ++i) {
        _ = rand() % 256;
        if (!t[_]) {
            t[_] = 1;
            hash[i] = _;
        }
    }

    map map_1;

    _ = map_generate(&map_1, 500, 500);
    if (_) {
        printf("failed to generate map\n");
    } else {
        printf("map generated\n");
    }
    _ = map_save(&map_1, "C_map_ini.txt");
    _ = waterfall(&map_1, 10, .3, .99);
    _ = map_save(&map_1, "C_map_end.txt");

    return 0;
}