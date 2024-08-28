#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define M_PI 3.14159265358979323846
int SEED;
double lost_over_map = 0;
const int SKIP_INITIAL_CYCLES = 1;
const int PERLIN_CYCLES = 2;
const double PERLIN_MODIF = .8;
const double PERLIN_HEIGHT_START = 50.0;
const double TARGET_HEIGHT = 50.0;
const int XN = 8;
const int YN = 8;
int64_t vectors_size;
int64_t* vectors;
const int XSIZE = (1 << XN);
const int YSIZE = (1 << YN);
const double SEEPING = .99;
// const double RAIN = 50000;
const double REMAINING_WATER_STOP = .1;
const double FALL_PUNISHMENT = -.1;
int expected_passes;

void fill_vectors()
{
    int64_t s = 1;
    int mx = XN;
    if (mx < YN)
        mx = YN;
    if (mx < SKIP_INITIAL_CYCLES + PERLIN_CYCLES)
        mx = SKIP_INITIAL_CYCLES + PERLIN_CYCLES;
    //    int mx = (SKIP_INITIAL_CYCLES + PERLIN_CYCLES > XN + YN) ? SKIP_INITIAL_CYCLES + PERLIN_CYCLES : XN + YN;
    for (int i = 0; i < mx; ++i)
        s *= 2;
    s = s * s;
    vectors = (int64_t*)malloc(sizeof(int64_t) * s);
    vectors_size = s;
    int64_t seed = time(NULL);
    printf("using seed %ld", seed);
    srand(seed);
    char taken[s];
    int64_t q = 0;
    for (int i = 0; i < s; ++i)
        taken[i] = 0;
    for (int i = 0; i < s; ++i) {
        q = rand() % s;
        while (taken[q] == 1) {
            q = rand() % s;
        }
        taken[q] = 1;
        vectors[i] = q;
    }
    printf("using hash table:\n");
    for (int i = 0; i < s; ++i)
        printf("%ld, ", vectors[i]);
    printf("\n");
    return;
}

// Constants
#define PERMUTATION_SIZE 256
#define FADE(t) ((t) * (t) * (t) * ((t) * ((t) * 6 - 15) + 10))
// Permutation table
int p[PERMUTATION_SIZE * 2];

// Gradient function

double noise_heuristic(double x, double y)
{
    printf("generating for %lf, %lf   ", x, y);
    x += 1.0;
    y += 1.0;
    int64_t xA = (int)x;
    int64_t xB = xA + 1;
    int64_t yA = (int)y;
    int64_t yC = yA + 1;
    printf("bounding box for (%.1lf|%.1lf) is    %ld %ld %ld %ld", x, y, xA, xB, yA, yC);

    double dA = 1 - sqrt((x - xA) * (x - xA) + (y - yA) * (y - yA));
    double dB = 1 - sqrt((x - xB) * (x - xB) + (y - yA) * (y - yA));
    double dC = 1 - sqrt((x - xB) * (x - xB) + (y - yC) * (y - yC));
    double dD = 1 - sqrt((x - xA) * (x - xA) + (y - yC) * (y - yC));
    dA = (dA > 0) ? dA : 0;
    dB = (dB > 0) ? dB : 0;
    dC = (dC > 0) ? dC : 0;
    dD = (dD > 0) ? dD : 0;
    double sd = dA + dB + dC + dD;
    dA /= sd;
    dB /= sd;
    dC /= sd;
    dD /= sd;
    /*
        double dA = (sqrt(2) - sqrt((x - xA) * (x - xA) + (y - yA) * (y - yA))) / sqrt(2);
        double dB = (sqrt(2) - sqrt((x - xB) * (x - xB) + (y - yA) * (y - yA))) / sqrt(2);
        double dC = (sqrt(2) - sqrt((x - xB) * (x - xB) + (y - yC) * (y - yC))) / sqrt(2);
        double dD = (sqrt(2) - sqrt((x - xA) * (x - xA) + (y - yC) * (y - yC))) / sqrt(2);
    */
    //    double sd = dA + dB + dC + dD;
    //    printf("d: %lf %lf %lf %lf   ", dA, dB, dC, dD);
    //    printf("generated %lf\n", sin((vectors[xA * XN + yA] * dA + vectors[xB * XN + yA] * dB + vectors[xB * XN + yC] * dC + vectors[xA * XN + yC] * dD) / sd * 2 * M_PI));
    //    return cos((vectors[xA * XN + yA] * dA + vectors[xB * XN + yA] * dB + vectors[xB * XN + yC] * dC + vectors[xA * XN + yC] * dD) / sd * 1.0 * M_PI);
    /*
    dA /= sd;
    dB /= sd;
    dC /= sd;
    dD /= sd;
    /**/
    printf("d: %lf %lf %lf %lf   ", dA, dB, dC, dD);
    //    int d = (x + y) * 2.0 / ((double)(XSIZE + YSIZE)) * PERMUTATION_SIZE;
    //    double fA = sin(((double)vectors[xA * XN + yA] / vectors_size) * 2 * M_PI) * dA;
    //    double fB = sin(((double)vectors[xB * XN + yA] / vectors_size) * 2 * M_PI) * dB;
    //    double fC = sin(((double)vectors[xB * XN + yC] / vectors_size) * 2 * M_PI) * dC;
    //    double fD = sin(((double)vectors[xA * XN + yC] / vectors_size) * 2 * M_PI) * dD;

    double fA = (double)vectors[xA * (1 << XN) + yA] / vectors_size * dA;
    double fB = (double)vectors[xB * (1 << XN) + yA] / vectors_size * dB;
    double fC = (double)vectors[xB * (1 << XN) + yC] / vectors_size * dC;
    double fD = (double)vectors[xA * (1 << XN) + yC] / vectors_size * dD;
    printf("generated %lf\n", fA + fB + fC + fD);
    return fA + fB + fC + fD;
    //    return sin(x + p[d]) + cos(y + p[d]);
    //    return sqrt((sin(xd) * cos(yd)) * (sin(xd) * cos(yd)));
}

double start_height(int64_t x, int64_t y)
{
    double xd = x;
    double yd = y;
    double q = 0;
    double m = PERLIN_HEIGHT_START;
    int md = (XN > YN) ? XN : YN;
    while (md) {
        --md;
        xd /= 2.0;
        yd /= 2.0;
    }
    for (int i = 0; i < SKIP_INITIAL_CYCLES; ++i) {
        xd *= 2.0;
        yd *= 2.0;
    }
    for (int i = 0; i < PERLIN_CYCLES; ++i) {
        q += noise_heuristic(xd, yd) * m;
        m *= PERLIN_MODIF;
        xd *= 2.0;
        yd *= 2.0;
    }
    return q;
}

double grad(int hash, double x, double y)
{
    int h = hash & 7; // Convert low 3 bits of hash code
    double u = h < 4 ? x : y; // Grad depends on h
    double v = h < 4 ? y : x;
    return ((h & 1) ? -u : u) + ((h & 2) ? -v : v);
}

// Linear interpolation
double lerp(double t, double a, double b)
{
    return a + t * (b - a);
}

// Perlin noise function
double perlin(double x, double y)
{
    // Find unit grid cell containing the point
    int X = (int)(x) & 255;
    int Y = (int)(y) & 255;

    // Relative coordinates in the grid cell
    x -= (int)x;
    y -= (int)y;

    // Compute fade curves for x and y
    double u = FADE(x);
    double v = FADE(y);

    // Hash coordinates of the 4 corners
    int a = p[X] + Y;
    int aa = p[a];
    int ab = p[a + 1];
    int b = p[X + 1] + Y;
    int ba = p[b];
    int bb = p[b + 1];

    // Add blended results from the corners
    double result = lerp(v, lerp(u, grad(p[aa], x, y), grad(p[ba], x - 1, y)),
        lerp(u, grad(p[ab], x, y - 1),
            grad(p[bb], x - 1, y - 1)));

    return sqrt((result + 1.0) / 2.0); // Normalize to [0, 1]
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
                    local.dirt[xo + 1][yo + 1] = dirt_levels->data[x + xo][y + yo + 1] + FALL_PUNISHMENT;
                    local.water[xo + 1][yo + 1] = water_levels->data[x + xo][y + yo + 1] + FALL_PUNISHMENT;
                } else {
                    local.dirt[xo + 1][yo + 1] = dirt_levels->data[x + xo][y + yo - 1] + FALL_PUNISHMENT;
                    local.water[xo + 1][yo + 1] = water_levels->data[x + xo][y + yo - 1] + FALL_PUNISHMENT;
                }
            } else if ((y + yo >= 0) && (y + yo < water_levels->size_y)) { // y ok x out of bounds
                if (x + xo < 0) {
                    local.dirt[xo + 1][yo + 1] = dirt_levels->data[x + xo + 1][y + yo] + FALL_PUNISHMENT;
                    local.water[xo + 1][yo + 1] = water_levels->data[x + xo + 1][y + yo] + FALL_PUNISHMENT;
                } else {
                    local.dirt[xo + 1][yo + 1] = dirt_levels->data[x + xo - 1][y + yo] + FALL_PUNISHMENT;
                    local.water[xo + 1][yo + 1] = water_levels->data[x + xo - 1][y + yo] + FALL_PUNISHMENT;
                }
            } else { // both out of bounds
                local.dirt[xo + 1][yo + 1] = dirt_levels->data[x][y] + FALL_PUNISHMENT;
                local.water[xo + 1][yo + 1] = water_levels->data[x][y] + FALL_PUNISHMENT;
            }
        }
    }

    return local;
}

// adds diff v
void paste_surroundings(map* water_levels, map* dirt_levels, local_stack v)
{
    for (int xo = 0; xo < 3; ++xo) {
        for (int yo = 0; yo < 3; ++yo) {
            if ((v.middlex + xo - 1 >= 0) && (v.middlex + xo - 1 < water_levels->size_x) && (v.middley + yo - 1 >= 0) && (v.middley + yo - 1 < water_levels->size_y)) {
                if (v.dirt[xo][yo] < 0) {
                    // printf("%d %d goes from %.lf to ", v.middlex, v.middley, dirt_levels->data[v.middlex][v.middley]);
                }
                water_levels->data[v.middlex + xo - 1][v.middley + yo - 1] += v.water[xo][yo];
                dirt_levels->data[v.middlex + xo - 1][v.middley + yo - 1] += v.dirt[xo][yo];
                if (v.dirt[xo][yo] < 0) {
                    // printf("to %.1lf\n", dirt_levels->data[xo][yo]);
                }
            } else {
                if (v.dirt[xo][yo] > 0) {
                    lost_over_map += v.dirt[xo][yo];
                    // printf("%.1lf lost over map on %d %d\n", v.dirt[xo][yo], v.middlex + xo - 1, v.middley + yo - 1);
                }
            }
        }
    }
    //    water_levels->data[v.middlex][v.middley] += v.water[1][1];
    //   dirt_levels->data[v.middlex][v.middley] += v.dirt[1][1];
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
    for (int i = 0; i < XSIZE; ++i)
        for (int j = 0; j < YSIZE; ++j) {
            water[0].data[i][j] = vectors[i * XSIZE + j];
            water[1].data[i][j] = vectors[i * XSIZE + j];
        }
    //    map_copy(self, water);
    map dirt[2];
    map_fill(dirt, 0, self->size_x, self->size_y);
    map_fill(dirt + 1, 0, self->size_x, self->size_y);
    map_copy(self, dirt + 1);
    map moves;
    map_fill(&moves, 0, self->size_x, self->size_y);

    int64_t i = 1;
    double water_initial = map_sweep(water);
    double water_remaining = water_initial;
    double local_average = 0;
    double diff;
    double dirt_initial = map_sweep(dirt + 1);
    double dirt_remaining;

    int which = 0;
    char p[] = "intermediate_.txt";

    while (water_remaining > water_initial * until) {

        //    for (; water_remaining > water_initial * until; i ^= 1) {
        map_copy(dirt + i, dirt + (i ^ 1));

        for (int x = 0; x < self->size_x; ++x)
            for (int y = 0; y < self->size_y; ++y)
                water[i].data[x][y] *= seeping;
        map_copy(water + i, water + (i ^ 1));

        water_remaining = map_sweep(water + i);
        dirt_remaining = map_sweep(dirt + i);
        if (dirt_remaining < 0) {
            printf("negative dirt remaining");
            map_save(dirt, "error.txt");
            return 1;
        }
        printf("water remaining: %.1lf aka %.3lf%%   ", water_remaining, water_remaining / water_initial * 100.0);
        printf("dirt remaining: %.1lf aka %.3lf%%    pass %d out of %d expected\n", dirt_remaining, dirt_remaining / dirt_initial * 100.0, which, expected_passes);

        for (int x = 0; x < self->size_x; ++x) {
            for (int y = 0; y < self->size_y; ++y) {
                local_stack here = get_surroundings(water + i, dirt + i, x, y);
                local_average = 0;
                int minx = 1;
                int miny = 1;
                local_stack change;
                change.middlex = x;
                change.middley = y;
                for (int xo = 0; xo < 3; ++xo)
                    for (int yo = 0; yo < 3; ++yo) {
                        if (here.dirt[xo][yo] + here.water[xo][yo] < here.dirt[minx][miny] + here.water[minx][miny]) {
                            minx = xo;
                            miny = yo;
                        }
                        change.dirt[xo][yo] = 0;
                        change.water[xo][yo] = 0;
                    }

                if (minx != 1 || miny != 1) {
                    double dirt_delta = 0;
                    double water_delta = 0;
                    if (here.dirt[1][1] > here.dirt[minx][miny]) {
                        dirt_delta = (here.dirt[1][1] + here.dirt[minx][miny]) / 2.0 - here.dirt[1][1];
                        // printf("dirt delta is %.1lf, from %.1lf to %.1lf\n", dirt_delta, here.dirt[1][1], here.dirt[minx][miny]);
                        change.dirt[1][1] = dirt_delta;
                        change.dirt[minx][miny] = -dirt_delta;
                        if (dirt_delta < 0) {
                            // printf("negative dirt delta %lf from (%d,%d) to (%d,%d)\n", dirt_delta, x, y, x + minx, y + miny);
                        }
                        paste_surroundings(&moves, &moves, change);
                        // moves.data[x][y] -= dirt_delta;
                        water_delta = (here.water[1][1] + here.water[minx][miny]) / 2.0 - here.water[1][1];
                        change.water[1][1] = water_delta;
                        change.water[minx][miny] = -water_delta;
                        paste_surroundings(water + (i ^ 1), dirt + (i ^ 1), change);
                    } else {
                        // water_delta = (here.dirt[1][1] + here.dirt[minx][miny]) / 2.0 - here.dirt[1][1];
                    }
                }
            }
        }

        p[12] = which % ('z' - 'a') + 'a';
        map_save(dirt + 1, p);
        ++which;
        i ^= 1;
    }
    map_copy(dirt + i, self);
    map_save(&moves, "changesmap.txt");

    return 0;
}

void map_pushup(map* self)
{
    double l = self->data[0][0];
    for (int i = 0; i < self->size_x; ++i)
        for (int j = 0; j < self->size_y; ++j)
            l = (l < self->data[i][j]) ? l : self->data[i][j];
    for (int i = 0; i < self->size_x; ++i)
        for (int j = 0; j < self->size_y; ++j)
            self->data[i][j] -= l;
    return;
}
void map_scale(map* self)
{
    double l = self->data[0][0];
    for (int i = 0; i < self->size_x; ++i)
        for (int j = 0; j < self->size_y; ++j)
            l = (l > self->data[i][j]) ? l : self->data[i][j];
    double modif = TARGET_HEIGHT / l;
    for (int i = 0; i < self->size_x; ++i)
        for (int j = 0; j < self->size_y; ++j)
            self->data[i][j] *= modif;
    return;
}

int main()
{
    fill_vectors();
    int64_t _;
    /*
        char t[PERMUTATION_SIZE * 2];
        for (int i = 0; i < PERMUTATION_SIZE * 2; ++i)
            t[i] = 0;
        int64_t _ = rand() % (PERMUTATION_SIZE * 2);
        for (int i = 0; i < PERMUTATION_SIZE * 2; ++i) {
            while (t[_])
                _ = rand() % (PERMUTATION_SIZE * 2);
            t[_] = 1;
            p[i] = _;
        }

        printf("using hash table: ");
        for (int i = 0; i < PERMUTATION_SIZE * 2; ++i)
            printf("%d,", p[i]);
        printf("\n");
    */

    expected_passes = 0;
    double q = 1;
    while (q > REMAINING_WATER_STOP && expected_passes < 10000) {
        ++expected_passes;
        q *= SEEPING;
    }
    printf("expecting %d passses\n", expected_passes);

    map map_1;

    _ = map_generate(&map_1, (1 << XN), (1 << YN));
    if (_) {
        printf("failed to generate map\n");
    } else {
        printf("map generated\n");
    }
    map_pushup(&map_1);
    map_scale(&map_1);
    double rain = 0;
    for (int x = 0; x < (1 << XN); ++x)
        for (int y = 0; y < (1 << YN); ++y)
            rain = (map_1.data[x][y] > rain) ? map_1.data[x][y] : rain;

    rain = rain / 10;

    _ = map_save(&map_1, "C_map_ini.txt");
    for (int i = 0; i < 2; ++i)
        _ = waterfall(&map_1, rain, REMAINING_WATER_STOP, SEEPING);
    _ = map_save(&map_1, "C_map_end.txt");

    free(vectors);
    return 0;
}