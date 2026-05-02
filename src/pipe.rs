use std::future::Future;
use std::pin::Pin;


pub struct Pipe<T> {
    value: T,
}

pub fn pipe<T>(value: T) -> Pipe<T> {
    Pipe { value }
}

impl<T> Pipe<T> {
    pub fn then<F, U>(self, f: F) -> Pipe<U>
    where
        F: FnOnce(T) -> U,
    {
        Pipe { value: f(self.value) }
    }

    pub fn inspect<F>(self, f: F) -> Self
    where
        F: FnOnce(&T),
    {
        f(&self.value);
        self
    }

    pub fn get(self) -> T {
        self.value
    }
}


impl<T, E> Pipe<Result<T, E>> {
    pub fn then_ok<F, U>(self, f: F) -> Pipe<Result<U, E>>
    where
        F: FnOnce(T) -> Result<U, E>,
    {
        Pipe {
            value: self.value.and_then(f),
        }
    }

    pub fn then_map<F, U>(self, f: F) -> Pipe<Result<U, E>>
    where
        F: FnOnce(T) -> U,
    {
        Pipe {
            value: self.value.map(f),
        }
    }

    pub fn catch<F>(self, f: F) -> Self
    where
        F: FnOnce(&E),
    {
        if let Err(ref e) = self.value {
            f(e);
        }
        self
    }
}


pub struct PipeAsync<T> {
    value: T,
}

pub fn pipe_async<T>(value: T) -> PipeAsync<T> {
    PipeAsync { value }
}

impl<T> PipeAsync<T> {
    pub async fn then<F, Fut, U>(self, f: F) -> PipeAsync<U>
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = U>,
    {
        PipeAsync {
            value: f(self.value).await,
        }
    }

    pub fn get(self) -> T {
        self.value
    }
}


impl<T> Pipe<Vec<T>> {
    pub fn map<F, U>(self, f: F) -> Pipe<Vec<U>>
    where
        F: Fn(T) -> U,
    {
        Pipe {
            value: self.value.into_iter().map(f).collect(),
        }
    }

    pub fn filter<F>(self, f: F) -> Pipe<Vec<T>>
    where
        F: Fn(&T) -> bool,
    {
        Pipe {
            value: self.value.into_iter().filter(f).collect(),
        }
    }

    pub async fn par_map<F, Fut, U>(self, f: F) -> Pipe<Vec<U>>
    where
        F: Fn(T) -> Fut + Copy,
        Fut: Future<Output = U>,
    {
        let futures: Vec<_> = self.value.into_iter().map(f).collect();
        let results = futures::future::join_all(futures).await;

        Pipe { value: results }
    }
}


pub struct Pipeline<T> {
    value: T,
    pipes: Vec<Box<dyn Fn(T) -> T>>,
}

pub fn pipeline<T>(value: T) -> Pipeline<T> {
    Pipeline {
        value,
        pipes: Vec::new(),
    }
}

impl<T: 'static> Pipeline<T> {
    pub fn through<F>(mut self, f: F) -> Self
    where
        F: Fn(T) -> T + 'static,
    {
        self.pipes.push(Box::new(f));
        self
    }

    pub fn run(self) -> T {
        let mut val = self.value;

        for pipe in self.pipes {
            val = pipe(val);
        }

        val
    }
}


pub struct PipelineAsync<T> {
    value: T,
    pipes: Vec<
        Box<
            dyn Fn(T) -> Pin<Box<dyn Future<Output = T> + Send>>
            + Send
        >
    >,
}

pub fn pipeline_async<T>(value: T) -> PipelineAsync<T> {
    PipelineAsync {
        value,
        pipes: Vec::new(),
    }
}

impl<T: Send + 'static> PipelineAsync<T> {
    pub fn through<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = T> + Send + 'static,
    {
        self.pipes.push(Box::new(move |val| {
            Box::pin(f(val))
        }));

        self
    }

    pub async fn run(self) -> T {
        let mut val = self.value;

        for pipe in self.pipes {
            val = pipe(val).await;
        }

        val
    }
}