use clap::Parser;
use std::collections::HashSet;
use std::fmt;

pub enum Cluster<T, U> {
    read_ids(HashSet<T>),
    mini_seqs(HashSet<U>),
}



#[derive(Clone)]
pub(crate) struct FastaRecord {
    //a struct used to store fasta records
    pub header: String,
    pub sequence: String,
}


impl fmt::Display for FastaRecord {
    // enables displaying the fasta record
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.header, self.sequence)
    }
}


pub(crate) struct FastqRecord {
    //a struct used to store fastq records
    header: String,
    sequence: String,
    quality_header: String,
    quality: String,
}
impl fmt::Display for FastqRecord {
    // enables displaying the fastq record
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.header, self.sequence)
    }
}


pub struct FastqRecord_isoncl_init {
    //a struct used to store fastq records
    pub header: String,
    pub internal_id: i32,
    pub sequence: String,
    //pub(crate) quality_header: String,
    pub quality: String,
    pub score: f64,
    pub error_rate:f64,
}


impl FastqRecord_isoncl_init{
    pub fn get_header(&self)->&str{
        &self.header
    }
    pub fn get_int_id(&self)->&i32{
        &self.internal_id
    }
    pub fn get_sequence(&self)->&str{
        &self.sequence
    }
    pub fn get_quality(&self)->&str{
        &self.quality
    }
    pub fn get_score(&self)->&f64{
        &self.score
    }
    pub fn get_err_rate(&self)->&f64{
        &self.error_rate
    }
    pub fn set_error_rate(&mut self, new_error_rate: f64){
        self.error_rate = new_error_rate
    }
    pub fn set_score(&mut self, new_score: f64) {
        self.score = new_score
    }
}


impl fmt::Display for FastqRecord_isoncl_init {
    // enables displaying the fastq record
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.header, self.sequence)
    }
}
