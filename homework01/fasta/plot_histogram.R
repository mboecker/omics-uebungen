fname = "p100.txt"
x = read.csv(fname, header = F)

n = x$V1
hash_n = x$V2

barplot(height = log(hash_n), names.arg = n)

# //////////////////////////////////////////////////////////////////////////

fname = "p10.txt"
x = read.csv(fname, header = F)

n = x$V1
hash_n = x$V2

barplot(height = log(hash_n), names.arg = n)

# //////////////////////////////////////////////////////////////////////////

fname = "p5.txt"
x = read.csv(fname, header = F)

n = x$V1
hash_n = x$V2

barplot(height = log(hash_n), names.arg = n)