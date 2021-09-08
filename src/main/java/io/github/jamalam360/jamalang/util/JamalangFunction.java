package io.github.jamalam360.jamalang.util;

@FunctionalInterface
public interface JamalangFunction<T> {
    T execute(String... args) throws Exception;
}
