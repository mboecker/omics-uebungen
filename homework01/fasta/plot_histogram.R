pdf("plots.pdf")

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

# //////////////////////////////////////////////////////////////////////////

fname = "data_without_canon.txt"
x = read.csv(fname, header = F)

p = x[[1]]
fpr = x[[2]]
fnr = x[[3]]

plot(  x = fpr,  y = fnr,  pch = 20,  type = "p",  xlim = c(0, 100),  ylim = c(0, 100))
#text(fpr,     fnr,     labels = p,     cex = 0.7,     pos = 4)

dev.off()